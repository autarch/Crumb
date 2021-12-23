use itertools::Itertools;
use lazy_regex::regex_is_match;
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub(crate) struct MBName {
    pub(crate) name: String,
    pub(crate) sort_name: Option<String>,
    pub(crate) name_type: String,
    pub(crate) locale: Option<String>,
}

#[derive(Debug)]
pub(crate) struct Name {
    pub(crate) name: String,
    pub(crate) sort_name: Option<String>,
    pub(crate) name_type: NameOrAliasType,
    pub(crate) locale: Option<String>,
}

#[derive(Debug)]
pub(crate) enum NameOrAliasType {
    PrimaryName, // The name column from the artist or release table.
    AliasName,   // An alias with "Artist name" or "Release name"
    SearchHint,
}

pub(crate) struct Names<'n> {
    pub(crate) name: &'n str,
    pub(crate) sortable_name: Option<&'n str>,
    pub(crate) transcripted_name: Option<&'n str>,
    pub(crate) transcripted_sortable_name: Option<&'n str>,
    pub(crate) translated_name: Option<&'n str>,
    pub(crate) translated_sortable_name: Option<&'n str>,
    pub(crate) search_hint_aliases: Vec<&'n str>,
}

#[derive(Clone, Debug)]
pub(crate) struct Alias<'a> {
    pub(crate) name: &'a str,
    pub(crate) sortable_name: Option<&'a str>,
    pub(crate) alias_type: AliasType,
    pub(crate) locale: Option<&'a str>,
    pub(crate) is_primary: bool,
}

#[derive(Clone, Debug)]
pub(crate) enum AliasType {
    Transcripted,
    Translated,
    SearchHint,
}

impl Into<Name> for MBName {
    fn into(self) -> Name {
        Name {
            name: self.name,
            sort_name: self.sort_name,
            name_type: match self.name_type.as_str() {
                "primary" => NameOrAliasType::PrimaryName,
                "alias" => NameOrAliasType::AliasName,
                "search" => NameOrAliasType::SearchHint,
                wtf => panic!("Unknown name type: {}", wtf),
            },
            locale: self.locale,
        }
    }
}

pub(crate) fn names_and_aliases<'n>(possible_names: &'n [Name]) -> Names<'n> {
    let (name, sortable_name) = determine_name_and_sortable_name(&possible_names);
    let aliases = determine_aliases(name, &possible_names);
    let (search_hints, rest): (Vec<Alias<'_>>, Vec<Alias<'_>>) = aliases
        .into_iter()
        .partition(|a| matches!(a.alias_type, AliasType::SearchHint));
    let (transcripted, translated): (Vec<Alias<'_>>, Vec<Alias<'_>>) = rest
        .into_iter()
        .partition(|a| matches!(a.alias_type, AliasType::Transcripted));

    let mut transcripted_iter = sorted_by_best_alias(transcripted);
    let transcripted_name = transcripted_iter.next();
    let mut translated_iter = sorted_by_best_alias(translated);
    let translated_name = translated_iter.next();

    Names {
        name,
        sortable_name,
        transcripted_name: transcripted_name.as_ref().map(|t| t.name),
        transcripted_sortable_name: transcripted_name
            .as_ref()
            .map(|t| t.sortable_name)
            .unwrap_or_else(|| transcripted_name.map(|t| t.name)),
        translated_name: translated_name.as_ref().map(|t| t.name),
        translated_sortable_name: translated_name
            .as_ref()
            .map(|t| t.sortable_name)
            .unwrap_or_else(|| translated_name.map(|t| t.name)),
        search_hint_aliases: transcripted_iter
            .chain(translated_iter)
            .chain(search_hints)
            .map(|a| a.name)
            .collect(),
    }
}

// We always treat the first primary name (meaning it came from the artist,
// release, or track table) as the correct name. That's because regardless of
// locale, we can expect to find names in various scripts. For example, many
// Japanese artists use English names in Latin script, like "the pillows",
// "Blankey Jet City", "BACK HORN", etc. While those artists may also have
// Japanese aliases, the "real" name of the artist is the English one.
fn determine_name_and_sortable_name<'n>(possible_names: &'n [Name]) -> (&'n str, Option<&'n str>) {
    let name = possible_names
        .iter()
        .find(|p| matches!(p.name_type, NameOrAliasType::PrimaryName))
        .unwrap();
    if is_latin(&name.name) {
        return (&name.name, possible_names[0].sort_name.as_deref());
    }
    // If the name is non-Latin, then the sortable name should be
    // too. However, MusicBrainz has a policy of always using Latin script for
    // sortable names. See https://musicbrainz.org/doc/Style/Artist/Sort_Name
    // for details.
    let sortable_name = match possible_names[0].sort_name.as_deref() {
        Some(sort_name) => {
            if is_latin(sort_name) {
                None
            } else {
                Some(sort_name)
            }
        }
        None => None,
    };
    (&name.name, sortable_name)
}

fn determine_aliases<'n>(name: &'n str, possible_names: &'n [Name]) -> Vec<Alias<'n>> {
    // We only care about Latin script aliases. For example, we don't need
    // Muse's name translated into Japanese, or the pillows for that matter.
    let names_with_known_word_counts: HashMap<&str, (u16, &Name)> = possible_names
        .iter()
        .filter(|n| {
            n.name != name
                && is_latin(&n.name)
                && match &n.sort_name {
                    Some(s) => is_latin(s),
                    None => true,
                }
        })
        .map(|n| (n.name.as_str(), (known_words_in_name(&n.name), n)))
        .collect();

    // If the name is in Latin script, then for our purposes all aliases are
    // search hints, not translations or transcriptions.
    let aliases = match names_with_known_word_counts.len() {
        0 => vec![],
        1 => {
            let mc = names_with_known_word_counts.values().next().unwrap();
            let alias_type = if is_latin(name) {
                AliasType::SearchHint
            } else if mc.0 > 0 {
                AliasType::Translated
            } else {
                AliasType::Transcripted
            };
            vec![alias_from_name(mc.1, alias_type)]
        }
        _ => {
            let max_known_words = names_with_known_word_counts
                .values()
                .map(|(c, _)| *c)
                .max()
                .unwrap();
            names_with_known_word_counts
                .values()
                .map(|(c, m)| {
                    // If there are multiple aliases we assume the one with
                    // the most known words is the only translation, as long
                    // as it has at least one known word. This accounts for
                    // cases like the band "東京事変". The translation is
                    // "Tokyo Incidents" and the transcription is "Tokyo
                    // Jihen". Both of those aliases have a known word,
                    // "Tokyo", but the translation has _more_ known words.
                    let alias_type = if is_latin(name) {
                        AliasType::SearchHint
                    } else if max_known_words > 0 && *c == max_known_words {
                        AliasType::Translated
                    } else {
                        AliasType::Transcripted
                    };
                    alias_from_name(m, alias_type)
                })
                .collect::<Vec<_>>()
        }
    };

    // If there are aliases that only differ from the name by casing we will
    // skip those.
    aliases
        .into_iter()
        .filter(|a| a.name.to_lowercase() != name.to_lowercase())
        .collect::<Vec<_>>()
}

static KNOWN_WORDS: Lazy<HashSet<String>> = Lazy::new(|| {
    let raw_words = include_str!("words.txt");
    raw_words.lines().map(|w| w.to_lowercase()).collect()
});

fn known_words_in_name(alias: &str) -> u16 {
    let mut count = 0;
    for word in alias.split_whitespace() {
        if KNOWN_WORDS.contains(&word.to_lowercase()) {
            count += 1;
        } else if word.contains('-') {
            // We want to check this for the benefit of names like
            // "Green-Yellow Society". While "green-yellow" isn't in our known
            // words list, "green" and "yellow" are.
            for subword in word.split('-') {
                if KNOWN_WORDS.contains(&subword.to_lowercase()) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn sorted_by_best_alias(aliases: Vec<Alias<'_>>) -> impl Iterator<Item = Alias<'_>> {
    aliases.into_iter().sorted_by(|a, b| {
        let a_is_en = alias_locale_is_en(a);
        let b_is_en = alias_locale_is_en(b);
        if a_is_en && b_is_en && a.is_primary != b.is_primary {
            return Ord::cmp(&b.is_primary, &a.is_primary);
        } else if a_is_en != b_is_en {
            return Ord::cmp(&b_is_en, &a_is_en);
        }

        let a_has_sortable = a.sortable_name.is_some();
        let b_has_sortable = b.sortable_name.is_some();
        if a_has_sortable != b_has_sortable {
            return Ord::cmp(&b_has_sortable, &a_has_sortable);
        }

        let a_len = a.name.len();
        let b_len = b.name.len();
        if a_len != b_len {
            return Ord::cmp(&a_len, &b_len);
        }

        Ord::cmp(a.name, b.name)
    })
}

fn alias_locale_is_en(alias: &Alias<'_>) -> bool {
    match alias.locale {
        Some("en") => true,
        _ => false,
    }
}

pub(crate) fn is_latin(text: &str) -> bool {
    regex_is_match!(r"\A[\p{Latin}&\P{L}]+\z", text)
}

fn alias_from_name(n: &Name, alias_type: AliasType) -> Alias<'_> {
    Alias {
        name: &n.name,
        sortable_name: n.sort_name.as_deref(),
        alias_type: match &n.name_type {
            NameOrAliasType::PrimaryName | NameOrAliasType::AliasName => alias_type,
            NameOrAliasType::SearchHint => AliasType::SearchHint,
        },
        locale: n.locale.as_deref(),
        is_primary: matches!(&n.name_type, NameOrAliasType::PrimaryName),
    }
}

pub(crate) fn maybe_uncomma_name(name: &str) -> (String, Option<String>) {
    if name.contains(", ") {
        let s: Vec<&str> = name.split(", ").collect();
        return (format!("{} {}", s[1], s[0]), Some(name.to_string()));
    }
    (name.to_string(), None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determine_name_and_sortable_name() {
        let possible = &[Name {
            name: String::from("Muse"),
            sort_name: Some(String::from("Muse")),
            name_type: NameOrAliasType::PrimaryName,
            locale: None,
        }];
        let (name, sortable_name) = super::determine_name_and_sortable_name(possible);
        assert_eq!(name, "Muse");
        assert_eq!(sortable_name, Some("Muse"));

        let possible = &[Name {
            name: String::from("Muse"),
            sort_name: None,
            name_type: NameOrAliasType::PrimaryName,
            locale: None,
        }];
        let (name, sortable_name) = super::determine_name_and_sortable_name(possible);
        assert_eq!(name, "Muse");
        assert_eq!(sortable_name, None);

        let possible = &[Name {
            name: String::from("ザ・ピロウズ"),
            sort_name: Some(String::from("ザ・ピロウズ")),
            name_type: NameOrAliasType::PrimaryName,
            locale: None,
        }];
        let (name, sortable_name) = super::determine_name_and_sortable_name(possible);
        assert_eq!(name, "ザ・ピロウズ");
        assert_eq!(sortable_name, Some("ザ・ピロウズ"));

        let possible = &[Name {
            name: String::from("ザ・ピロウズ"),
            sort_name: None,
            name_type: NameOrAliasType::PrimaryName,
            locale: None,
        }];
        let (name, sortable_name) = super::determine_name_and_sortable_name(possible);
        assert_eq!(name, "ザ・ピロウズ");
        assert_eq!(sortable_name, None);

        let possible = &[Name {
            name: String::from("ザ・ピロウズ"),
            sort_name: Some(String::from("the pillows")),
            name_type: NameOrAliasType::PrimaryName,
            locale: None,
        }];
        let (name, sortable_name) = super::determine_name_and_sortable_name(possible);
        assert_eq!(name, "ザ・ピロウズ");
        assert_eq!(sortable_name, None);

        let possible = &[
            Name {
                name: String::from("ザ・ピロウズ"),
                sort_name: Some(String::from("Whatever")),
                name_type: NameOrAliasType::AliasName,
                locale: None,
            },
            Name {
                name: String::from("ザ・ピロウズ"),
                sort_name: Some(String::from("the pillows")),
                name_type: NameOrAliasType::PrimaryName,
                locale: None,
            },
        ];
        let (name, sortable_name) = super::determine_name_and_sortable_name(possible);
        assert_eq!(name, "ザ・ピロウズ");
        assert_eq!(sortable_name, None);
    }

    #[test]
    fn determine_aliases() {
        let possible: Vec<Name> = vec![Name {
            name: String::from("Muse"),
            sort_name: None,
            name_type: NameOrAliasType::AliasName,
            locale: None,
        }];
        let aliases = super::determine_aliases("Muse", &possible);
        assert!(
            aliases.is_empty(),
            "Aliases that match primary name are ignored"
        );

        let possible: Vec<Name> = vec![Name {
            name: String::from("ミューズ"),
            sort_name: None,
            name_type: NameOrAliasType::AliasName,
            locale: None,
        }];
        let aliases = super::determine_aliases("Muse", &possible);
        assert!(
            aliases.is_empty(),
            "Non-Latin aliases are ignored when primary name is Latin script"
        );

        let possible: Vec<Name> = vec![Name {
            name: String::from("The Muse"),
            sort_name: None,
            name_type: NameOrAliasType::AliasName,
            locale: None,
        }];
        let aliases = super::determine_aliases("Muse", &possible);
        assert_eq!(
            aliases.len(),
            1,
            "One Latin alias is returned when primary name is in Latin script"
        );
        assert_eq!(aliases[0].name, "The Muse");
        assert_eq!(aliases[0].sortable_name, None);
        assert!(matches!(aliases[0].alias_type, AliasType::SearchHint));

        let possible: Vec<Name> = vec![
            Name {
                name: String::from("The Muse"),
                sort_name: None,
                name_type: NameOrAliasType::AliasName,
                locale: None,
            },
            Name {
                name: String::from("Amuse Bouche"),
                sort_name: None,
                name_type: NameOrAliasType::AliasName,
                locale: None,
            },
        ];
        let mut aliases = super::determine_aliases("Muse", &possible);
        aliases.sort_by(|a, b| Ord::cmp(a.name, b.name));
        assert_eq!(
            aliases.len(),
            2,
            "Two Latin aliases are returned when primary name is in Latin script"
        );
        assert_eq!(aliases[0].name, "Amuse Bouche");
        assert_eq!(aliases[0].sortable_name, None);
        assert!(matches!(aliases[0].alias_type, AliasType::SearchHint));
        assert_eq!(aliases[1].name, "The Muse");
        assert_eq!(aliases[1].sortable_name, None);
        assert!(matches!(aliases[1].alias_type, AliasType::SearchHint));

        let possible: Vec<Name> = vec![Name {
            name: String::from("Green-Yellow Society"),
            sort_name: None,
            name_type: NameOrAliasType::AliasName,
            locale: None,
        }];
        let aliases = super::determine_aliases("緑黄色社会", &possible);
        assert_eq!(
            aliases.len(),
            1,
            "One Latin alias for Japanese name is returned"
        );
        assert_eq!(aliases[0].name, "Green-Yellow Society");
        assert_eq!(aliases[0].sortable_name, None);
        assert!(matches!(aliases[0].alias_type, AliasType::Translated));

        let possible: Vec<Name> = vec![
            Name {
                name: String::from("Green-Yellow Society"),
                sort_name: None,
                name_type: NameOrAliasType::AliasName,
                locale: None,
            },
            Name {
                name: String::from("Ryokushaka"),
                sort_name: None,
                name_type: NameOrAliasType::AliasName,
                locale: None,
            },
        ];
        let mut aliases = super::determine_aliases("緑黄色社会", &possible);
        aliases.sort_by(|a, b| Ord::cmp(a.name, b.name));
        assert_eq!(
            aliases.len(),
            2,
            "Two Latin aliases for Japanese name are returned"
        );
        assert_eq!(aliases[0].name, "Green-Yellow Society");
        assert_eq!(aliases[0].sortable_name, None);
        assert!(matches!(aliases[0].alias_type, AliasType::Translated));
        assert_eq!(aliases[1].name, "Ryokushaka");
        assert_eq!(aliases[1].sortable_name, None);
        assert!(matches!(aliases[1].alias_type, AliasType::Transcripted));

        let possible: Vec<Name> = vec![
            Name {
                name: String::from("Green-Yellow Society"),
                sort_name: None,
                name_type: NameOrAliasType::AliasName,
                locale: None,
            },
            Name {
                name: String::from("Ryokushaka"),
                sort_name: None,
                name_type: NameOrAliasType::AliasName,
                locale: None,
            },
            Name {
                name: String::from("Ryokuoushokushakai"),
                sort_name: None,
                name_type: NameOrAliasType::AliasName,
                locale: None,
            },
        ];
        let mut aliases = super::determine_aliases("緑黄色社会", &possible);
        aliases.sort_by(|a, b| Ord::cmp(a.name, b.name));
        assert_eq!(
            aliases.len(),
            3,
            "Three Latin aliases for Japanese name are returned"
        );
        assert_eq!(aliases[0].name, "Green-Yellow Society");
        assert_eq!(aliases[0].sortable_name, None);
        assert!(matches!(aliases[0].alias_type, AliasType::Translated));
        assert_eq!(aliases[1].name, "Ryokuoushokushakai");
        assert_eq!(aliases[1].sortable_name, None);
        assert!(matches!(aliases[1].alias_type, AliasType::Transcripted));
        assert_eq!(aliases[2].name, "Ryokushaka");
        assert_eq!(aliases[2].sortable_name, None);
        assert!(matches!(aliases[2].alias_type, AliasType::Transcripted));

        let possible = tokyo_incidents_names();
        let mut aliases = super::determine_aliases("東京事変", &possible);
        aliases.sort_by(|a, b| Ord::cmp(a.name, b.name));
        assert_eq!(
            aliases.len(),
            2,
            "Two Latin aliases for Japanese name are returned from list with duplicates"
        );
        assert_eq!(aliases[0].name, "Tokyo Incidents");
        assert_eq!(aliases[0].sortable_name, None);
        assert!(matches!(aliases[0].alias_type, AliasType::Translated));
        assert_eq!(aliases[1].name, "Tokyo Jihen");
        assert_eq!(aliases[1].sortable_name, None);
        assert!(matches!(aliases[1].alias_type, AliasType::Transcripted));
    }

    #[test]
    fn names_and_aliases() {
        let possible = tokyo_incidents_names();
        let names = super::names_and_aliases(&possible);
        assert_eq!(names.name, "東京事変");
        assert_eq!(names.sortable_name, None);
        assert_eq!(names.transcripted_name, Some("Tokyo Jihen"));
        assert_eq!(names.transcripted_sortable_name, None);
        assert_eq!(names.translated_name, Some("Tokyo Incidents"));
        assert_eq!(names.translated_sortable_name, None);
        assert_eq!(names.search_hint_aliases.len(), 0);

        let possible = kenichi_asai_names();
        let names = super::names_and_aliases(&possible);
        assert_eq!(names.name, "浅井健一");
        assert_eq!(names.sortable_name, None);
        assert_eq!(names.transcripted_name, Some("Kenichi Asai"));
        assert_eq!(names.transcripted_sortable_name, Some("Asai, Kenichi"));
        assert_eq!(names.translated_name, None);
        assert_eq!(names.translated_sortable_name, None);
        assert_eq!(
            names.search_hint_aliases,
            vec!["Asai Kenichi", "Asai Ken'ichi"],
        );
    }

    fn tokyo_incidents_names() -> Vec<Name> {
        vec![
            Name {
                name: String::from("東京事変"),
                sort_name: Some(String::from("Tokyo Jihen")),
                name_type: NameOrAliasType::PrimaryName,
                locale: None,
            },
            Name {
                name: String::from("Tokyo Jihen"),
                sort_name: Some(String::from("Tokyo Jihen")),
                name_type: NameOrAliasType::AliasName,
                locale: None,
            },
            Name {
                name: String::from("東京事変"),
                sort_name: None,
                name_type: NameOrAliasType::PrimaryName,
                locale: None,
            },
            Name {
                name: String::from("Tokyo Jihen"),
                sort_name: None,
                name_type: NameOrAliasType::PrimaryName,
                locale: None,
            },
            Name {
                name: String::from("東京事變"),
                sort_name: None,
                name_type: NameOrAliasType::PrimaryName,
                locale: None,
            },
            Name {
                name: String::from("Tokyo Incidents"),
                sort_name: None,
                name_type: NameOrAliasType::PrimaryName,
                locale: None,
            },
        ]
    }

    fn kenichi_asai_names() -> Vec<Name> {
        vec![
            Name {
                name: String::from("浅井健一"),
                sort_name: Some(String::from("Asai, Kenichi")),
                name_type: NameOrAliasType::PrimaryName,
                locale: None,
            },
            Name {
                name: String::from("浅井 健一"),
                sort_name: Some(String::from("あさい けんいち")),
                name_type: NameOrAliasType::AliasName,
                locale: Some(String::from("ja")),
            },
            Name {
                name: String::from("Benzie"),
                sort_name: Some(String::from("ベンジー")),
                name_type: NameOrAliasType::AliasName,
                locale: None,
            },
            Name {
                name: String::from("浅井健一"),
                sort_name: Some(String::from("あさいけんいち")),
                name_type: NameOrAliasType::PrimaryName,
                locale: Some(String::from("ja")),
            },
            Name {
                name: String::from("Asai Ken'ichi"),
                sort_name: Some(String::from("Asai Ken'ichi")),
                name_type: NameOrAliasType::SearchHint,
                locale: None,
            },
            Name {
                name: String::from("Kenichi Asai"),
                sort_name: Some(String::from("Asai, Kenichi")),
                name_type: NameOrAliasType::PrimaryName,
                locale: Some(String::from("en")),
            },
            Name {
                name: String::from("浅井健一"),
                sort_name: None,
                name_type: NameOrAliasType::AliasName,
                locale: None,
            },
            Name {
                name: String::from("Kenichi Asai"),
                sort_name: None,
                name_type: NameOrAliasType::AliasName,
                locale: None,
            },
            Name {
                name: String::from("Kenichi Asai"),
                sort_name: None,
                name_type: NameOrAliasType::AliasName,
                locale: None,
            },
            Name {
                name: String::from("Asai Kenichi"),
                sort_name: None,
                name_type: NameOrAliasType::AliasName,
                locale: None,
            },
            Name {
                name: String::from("Kenichi Asai"),
                sort_name: Some(String::from("Asai, Kenichi")),
                name_type: NameOrAliasType::AliasName,
                locale: None,
            },
            Name {
                name: String::from("ベンジー"),
                sort_name: None,
                name_type: NameOrAliasType::AliasName,
                locale: None,
            },
            Name {
                name: String::from("あさいけんいち"),
                sort_name: None,
                name_type: NameOrAliasType::AliasName,
                locale: None,
            },
            Name {
                name: String::from("あさい けんいち"),
                sort_name: None,
                name_type: NameOrAliasType::AliasName,
                locale: None,
            },
        ]
    }

    #[test]
    fn is_latin() {
        let yes = [
            "Dave",
            "Tokyo Incidents",
            "Péter",
            "Hildur Guðnadóttir",
            "Peter 22",
            "Blink 451",
            "1-1",
            "!()!",
        ];
        for name in yes {
            assert!(super::is_latin(name), r#""{}" is latin"#, name);
        }

        let no = ["（´・д・）ﾉ", "Пётр Ильич Чайковский", "布袋寅泰‎"];
        for name in no {
            assert!(!super::is_latin(name), r#""{}" is not latin"#, name);
        }
    }
}

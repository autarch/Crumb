use itertools::Itertools;
use lazy_regex::regex_is_match;
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub(crate) struct MBName {
    pub(crate) name: String,
    pub(crate) sort_name: Option<String>,
    pub(crate) name_type: String,
}

#[derive(Debug)]
pub(crate) struct Name {
    pub(crate) name: String,
    pub(crate) sort_name: Option<String>,
    pub(crate) name_type: NameOrAliasType,
}

#[derive(Debug)]
pub(crate) enum NameOrAliasType {
    PrimaryName, // The name column from the artist or release table.
    AliasName,   // An alias with "Artist name" or "Release name"
    SearchHint,
}

#[derive(Debug)]
pub(crate) struct Names<'n> {
    pub(crate) name: &'n str,
    pub(crate) sortable_name: Option<&'n str>,
    pub(crate) transcripted: Vec<Alias<'n>>,
    pub(crate) translated: Vec<Alias<'n>>,
    pub(crate) search_hint: Vec<Alias<'n>>,
}

#[derive(Debug)]
pub(crate) struct Alias<'a> {
    pub(crate) name: &'a str,
    pub(crate) sortable_name: Option<&'a str>,
    pub(crate) alias_type: AliasType,
}

#[derive(Debug)]
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
        }
    }
}

pub(crate) fn names_and_aliases<'n>(possible_names: &'n [Name]) -> Names<'n> {
    let (name, sortable_name) = determine_name_and_sortable_name(&possible_names);
    let aliases = determine_aliases(name, &possible_names);
    let (search_hint, rest): (Vec<Alias<'_>>, Vec<Alias<'_>>) = aliases
        .into_iter()
        .partition(|a| matches!(a.alias_type, AliasType::SearchHint));
    let (transcripted, translated): (Vec<Alias<'_>>, Vec<Alias<'_>>) = rest
        .into_iter()
        .partition(|a| matches!(a.alias_type, AliasType::Transcripted));
    Names {
        name,
        sortable_name,
        transcripted: sorted_by_size(transcripted),
        translated: sorted_by_size(translated),
        search_hint: sorted_by_size(search_hint),
    }
}

fn sorted_by_size(aliases: Vec<Alias<'_>>) -> Vec<Alias<'_>> {
    aliases
        .into_iter()
        .sorted_by(|a, b| Ord::cmp(&a.name.len(), &b.name.len()))
        .collect()
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
    let aliases_with_known_word_counts: HashMap<&str, (u16, &Name)> = possible_names
        .iter()
        .filter(|n| n.name != name && is_latin(&n.name))
        .map(|n| (n.name.as_str(), (known_words_in_name(&n.name), n)))
        .collect();

    // If the name is in Latin script, then for our purposes all aliases are
    // search hints, not translations or transcriptions.
    let mut aliases = match aliases_with_known_word_counts.len() {
        0 => vec![],
        1 => {
            let mc = aliases_with_known_word_counts.values().next().unwrap();
            let alias_type = if is_latin(name) {
                AliasType::SearchHint
            } else if mc.0 > 0 {
                AliasType::Translated
            } else {
                AliasType::Transcripted
            };
            vec![alias_from_mb_name(mc.1, alias_type)]
        }
        _ => {
            let max_known_words = aliases_with_known_word_counts
                .values()
                .map(|(c, _)| *c)
                .max()
                .unwrap();
            aliases_with_known_word_counts
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
                    alias_from_mb_name(m, alias_type)
                })
                .collect::<Vec<_>>()
        }
    };

    // If this is an artist with a non-latin name and a transcription or
    // translation as the sortable name, then it's possible that the only
    // occurrence of a particular alias is that sortable name, so we need to
    // include that too. The sortable name is always empty for releases.
    if !is_latin(name) {
        if let Some(sortable_name) = possible_names
            .iter()
            .find(|n| {
                if n.name == name {
                    if let Some(sn) = &n.sort_name {
                        return is_latin(sn);
                    }
                }
                false
            })
            .map(|n| n.sort_name.as_deref().unwrap())
        {
            aliases.push(Alias {
                name: sortable_name,
                sortable_name: None,
                alias_type: if known_words_in_name(sortable_name) == 0 {
                    AliasType::Transcripted
                } else {
                    AliasType::Translated
                },
            });
        }
    }

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

pub(crate) fn is_latin(text: &str) -> bool {
    regex_is_match!(r"\A[\p{Latin}&\P{L}]+\z", text)
}

fn alias_from_mb_name(n: &Name, alias_type: AliasType) -> Alias<'_> {
    Alias {
        name: &n.name,
        sortable_name: n.sort_name.as_deref(),
        alias_type: match &n.name_type {
            NameOrAliasType::PrimaryName | NameOrAliasType::AliasName => alias_type,
            NameOrAliasType::SearchHint => AliasType::SearchHint,
        },
    }
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
        }];
        let (name, sortable_name) = super::determine_name_and_sortable_name(possible);
        assert_eq!(name, "Muse");
        assert_eq!(sortable_name, Some("Muse"));

        let possible = &[Name {
            name: String::from("Muse"),
            sort_name: None,
            name_type: NameOrAliasType::PrimaryName,
        }];
        let (name, sortable_name) = super::determine_name_and_sortable_name(possible);
        assert_eq!(name, "Muse");
        assert_eq!(sortable_name, None);

        let possible = &[Name {
            name: String::from("ザ・ピロウズ"),
            sort_name: Some(String::from("ザ・ピロウズ")),
            name_type: NameOrAliasType::PrimaryName,
        }];
        let (name, sortable_name) = super::determine_name_and_sortable_name(possible);
        assert_eq!(name, "ザ・ピロウズ");
        assert_eq!(sortable_name, Some("ザ・ピロウズ"));

        let possible = &[Name {
            name: String::from("ザ・ピロウズ"),
            sort_name: None,
            name_type: NameOrAliasType::PrimaryName,
        }];
        let (name, sortable_name) = super::determine_name_and_sortable_name(possible);
        assert_eq!(name, "ザ・ピロウズ");
        assert_eq!(sortable_name, None);

        let possible = &[Name {
            name: String::from("ザ・ピロウズ"),
            sort_name: Some(String::from("the pillows")),
            name_type: NameOrAliasType::PrimaryName,
        }];
        let (name, sortable_name) = super::determine_name_and_sortable_name(possible);
        assert_eq!(name, "ザ・ピロウズ");
        assert_eq!(sortable_name, None);

        let possible = &[
            Name {
                name: String::from("ザ・ピロウズ"),
                sort_name: Some(String::from("Whatever")),
                name_type: NameOrAliasType::AliasName,
            },
            Name {
                name: String::from("ザ・ピロウズ"),
                sort_name: Some(String::from("the pillows")),
                name_type: NameOrAliasType::PrimaryName,
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
            },
            Name {
                name: String::from("Amuse Bouche"),
                sort_name: None,
                name_type: NameOrAliasType::AliasName,
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
            },
            Name {
                name: String::from("Ryokushaka"),
                sort_name: None,
                name_type: NameOrAliasType::AliasName,
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
            },
            Name {
                name: String::from("Ryokushaka"),
                sort_name: None,
                name_type: NameOrAliasType::AliasName,
            },
            Name {
                name: String::from("Ryokuoushokushakai"),
                sort_name: None,
                name_type: NameOrAliasType::AliasName,
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

        let possible: Vec<Name> = vec![
            Name {
                name: String::from("Green-Yellow Society"),
                sort_name: None,
                name_type: NameOrAliasType::AliasName,
            },
            Name {
                name: String::from("緑黄色社会"),
                sort_name: Some(String::from("Ryokuoushokushakai")),
                name_type: NameOrAliasType::PrimaryName,
            },
        ];
        let mut aliases = super::determine_aliases("緑黄色社会", &possible);
        aliases.sort_by(|a, b| Ord::cmp(a.name, b.name));
        assert_eq!(
            aliases.len(),
            2,
            "Two Latin aliases for Japanese name are returned, including one from Latin script sortable name"
        );
        assert_eq!(aliases[0].name, "Green-Yellow Society");
        assert_eq!(aliases[0].sortable_name, None);
        assert!(matches!(aliases[0].alias_type, AliasType::Translated));
        assert_eq!(aliases[1].name, "Ryokuoushokushakai");
        assert_eq!(aliases[1].sortable_name, None);
        assert!(matches!(aliases[1].alias_type, AliasType::Transcripted));
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

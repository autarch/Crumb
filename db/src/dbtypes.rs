use sqlx::{
    postgres::{PgTypeInfo, PgTypeKind},
    Decode, Encode, FromRow, Postgres, Type, TypeInfo,
};
use uuid::Uuid;

#[derive(Clone, Debug, Type)]
#[sqlx(type_name = "name_type", rename_all = "snake_case")]
pub enum NameType {
    Original,
    Transcripted,
    Translated,
}

// This wackiness is needed to work around an issue with a plain Vec<EnumType>
// - see https://github.com/launchbadge/sqlx/pull/1170#issuecomment-817738085.
#[derive(Clone, Debug, Decode, Encode)]
pub struct NameTypes(pub Vec<NameType>);

impl Type<Postgres> for NameTypes {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_name_type")
    }
}

#[derive(Debug, sqlx::Decode, sqlx::Encode)]
pub struct CiText(String);

impl sqlx::Type<sqlx::Postgres> for CiText {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("crumb.citext")
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        *ty == Self::type_info() || is_stringish(ty)
    }
}

impl CiText {
    pub fn into_string(self) -> String {
        self.0
    }
}

#[derive(Debug, sqlx::Decode, sqlx::Encode)]
pub struct Email(String);

impl sqlx::Type<sqlx::Postgres> for Email {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("crumb.email")
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        *ty == Self::type_info() || is_stringish(ty)
    }
}

fn is_stringish(ty: &PgTypeInfo) -> bool {
    if <&str as Type<Postgres>>::compatible(ty) {
        return true;
    }

    let name = match ty.kind() {
        PgTypeKind::Simple => ty.name(),
        PgTypeKind::Domain(d) => d.name(),
        _ => "",
    };
    matches!(name.to_uppercase().as_str(), "CITEXT" | "EMAIL",)
}

#[derive(Debug)]
pub struct TrackInfo {
    pub position: i32,
    pub title: String,
    pub album: String,
    pub artist: String,
}

#[derive(Debug, FromRow)]
pub struct TrackMatch {
    pub track_id: i32,
    pub position: i32,
    pub track_title: String,
    pub length: Option<i32>,
    pub release_id: i32,
    pub release_group_id: i32,
    pub release_date: Vec<Option<i16>>,
    pub original_release_date: Option<Vec<Option<i16>>>,
    pub album_title: String,
    pub release_comment: String,
    pub artist_id: i32,
    pub artist: String,
    pub artist_type: Option<String>,
    pub recording_id: i32,
    pub recording_gid: Uuid,
}

#[derive(Debug, FromRow)]
pub struct Artist {
    pub artist_id: Uuid,
    pub musicbrainz_artist_id: Option<i32>,
    pub name: CiText,
    pub sortable_name: CiText,
}

#[derive(Debug, FromRow)]
pub struct Release {
    pub release_id: Uuid,
    pub musicbrainz_release_id: Option<i32>,
    pub title: CiText,
    pub release_year: Option<i16>,
    pub release_month: Option<i16>,
    pub release_day: Option<i16>,
    pub original_year: Option<i16>,
    pub original_month: Option<i16>,
    pub original_day: Option<i16>,
}

#[derive(Debug, FromRow)]
pub struct ArtistItem {
    pub artist_id: Uuid,
    pub display_name: CiText,
    pub name: CiText,
    pub sortable_name: Option<CiText>,
    pub transcripted_name: Option<CiText>,
    pub transcripted_sortable_name: Option<CiText>,
    pub translated_name: Option<CiText>,
    pub translated_sortable_name: Option<CiText>,
    pub release_count: i64,
    pub track_count: i64,
    pub album_cover_uri: Option<String>,
}

#[derive(Debug, FromRow)]
pub struct ReleaseItem {
    pub release_id: Uuid,
    pub primary_artist_id: Uuid,
    pub display_title: CiText,
    pub title: CiText,
    pub transcripted_title: Option<CiText>,
    pub translated_title: Option<CiText>,
    pub comment: Option<CiText>,
    pub track_count: i64,
    pub release_year: Option<i16>,
    pub release_month: Option<i16>,
    pub release_day: Option<i16>,
    pub original_year: Option<i16>,
    pub original_month: Option<i16>,
    pub original_day: Option<i16>,
    pub album_cover_uri: Option<String>,
}

#[derive(Debug, FromRow)]
pub struct ReleaseTrack {
    pub track_id: Uuid,
    pub primary_artist_id: Uuid,
    pub display_title: CiText,
    pub title: CiText,
    pub transcripted_title: Option<CiText>,
    pub translated_title: Option<CiText>,
    pub length: Option<i32>,
    pub content_hash: String,
    pub position: i32,
}

#[derive(Debug)]
pub enum SortableThing {
    Artist,
    Release,
    Track,
}

#[derive(Debug, FromRow)]
pub struct User {
    pub user_id: Uuid,
    #[allow(dead_code)]
    pub email: Email,
    #[allow(dead_code)]
    pub date_format: String,
    pub preferred_name_order: NameTypes,
}

impl User {
    pub fn sort_order(&self, table: &str, what: SortableThing) -> String {
        let mut sort_order: Vec<&str> = vec![];
        for pn in &self.preferred_name_order.0 {
            match pn {
                NameType::Original => {
                    if matches!(what, SortableThing::Artist) {
                        sort_order.append(&mut vec!["sortable_name", "name"]);
                    } else {
                        sort_order.push("title");
                    }
                }
                NameType::Transcripted => {
                    if matches!(what, SortableThing::Artist) {
                        sort_order
                            .append(&mut vec!["transcripted_sortable_name", "transcripted_name"]);
                    } else {
                        sort_order.push("transcripted_title");
                    }
                }
                NameType::Translated => {
                    if matches!(what, SortableThing::Artist) {
                        sort_order.append(&mut vec!["translated_sortable_name", "translated_name"]);
                    } else {
                        sort_order.push("translated_title");
                    }
                }
            }
        }

        sort_order
            .iter()
            .map(|col| format!("{}.{}", table, col))
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub fn display_order(&self, table: &str, what: SortableThing) -> String {
        let col = if matches!(what, SortableThing::Artist) {
            "name"
        } else {
            "title"
        };
        self.preferred_name_order
            .0
            .iter()
            .map(|pn| match pn {
                NameType::Original => format!("{}.{}", table, col),
                NameType::Transcripted => format!("{}.transcripted_{}", table, col),
                NameType::Translated => format!("{}.translated_{}", table, col),
            })
            .collect::<Vec<_>>()
            .join(", ")
    }
}

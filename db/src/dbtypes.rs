use sqlx::{
    postgres::{PgTypeInfo, PgTypeKind},
    Decode, Encode, FromRow, Postgres, Type, TypeInfo,
};
use uuid::Uuid;

#[derive(Debug, Type)]
#[sqlx(type_name = "name_type", rename_all = "snake_case")]
pub enum NameType {
    Original,
    Transcripted,
    Translated,
}

// This wackiness is needed to work around an issue with a plain Vec<EnumType>
// - see https://github.com/launchbadge/sqlx/pull/1170#issuecomment-817738085.
#[derive(Debug, Decode, Encode)]
pub struct NameTypes(Vec<NameType>);

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
    matches!(
        name.to_uppercase().as_str(),
        "CITEXT" | "EMAIL",
    )
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
pub struct User {
    pub user_id: Uuid,
    #[allow(dead_code)]
    email: Email,
    #[allow(dead_code)]
    date_format: String,
    #[allow(dead_code)]
    preferred_name_order: NameTypes,
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

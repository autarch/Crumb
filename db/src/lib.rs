mod names;

use crate::names::*;
use itertools::Itertools;
use log::debug;
use sqlx::{
    postgres::{PgPool, PgPoolOptions},
    Postgres, Transaction,
};
use std::collections::{HashMap, HashSet};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum DBError {
    #[error(transparent)]
    SQLXError(#[from] sqlx::Error),
    #[error("Called insert_user_tracks for matches that match multiple artists")]
    CannotInsertUserTracksForMultipleArtists,
    #[error("Called insert_user_tracks for matches that match multiple releases")]
    CannotInsertUserTracksForMultipleReleases,
}

pub type DBResult<T> = Result<T, DBError>;

#[derive(Clone, Debug)]
pub struct DB {
    pool: PgPool,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "name_type", rename_all = "snake_case")]
pub enum NameType {
    Original,
    Transcripted,
    Translated,
}

// This wackiness is needed to work around an issue with a plain Vec<EnumType>
// - see https://github.com/launchbadge/sqlx/pull/1170#issuecomment-817738085.
#[derive(Debug, sqlx::Decode, sqlx::Encode)]
pub struct NameTypes(Vec<NameType>);

impl sqlx::Type<sqlx::Postgres> for NameTypes {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_name_type")
    }
}

#[derive(Debug)]
pub struct TrackInfo {
    pub position: i32,
    pub title: String,
    pub album: String,
    pub artist: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct TrackMatch {
    pub track_id: i32,
    pub position: i32,
    pub track_title: String,
    pub length: Option<i32>,
    pub release_id: i32,
    pub release_group_id: i32,
    release_date: Vec<Option<i16>>,
    original_release_date: Option<Vec<Option<i16>>>,
    pub album_title: String,
    pub release_comment: String,
    pub artist_id: i32,
    pub artist: String,
    pub recording_id: i32,
    pub recording_gid: Uuid,
}

#[derive(Debug)]
pub struct User {
    pub user_id: Uuid,
    email: String,
    date_format: String,
    preferred_name_order: NameTypes,
}

#[derive(Debug)]
pub struct Artist {
    artist_id: Uuid,
    musicbrainz_artist_id: Option<i32>,
    name: String,
    sortable_name: String,
}

#[derive(Debug)]
pub struct Release {
    release_id: Uuid,
    musicbrainz_release_id: Option<i32>,
    title: String,
    release_year: Option<i16>,
    release_month: Option<i16>,
    release_day: Option<i16>,
    original_year: Option<i16>,
    original_month: Option<i16>,
    original_day: Option<i16>,
}

#[derive(Debug)]
pub struct Track {
    track_id: Uuid,
    musicbrainz_id: Option<i32>,
    primary_artist_id: Uuid,
    title: String,
    length: i16,
    storage_uri: String,
}

const TRACK_MATCH_SELECT: &str = r#"
SELECT DISTINCT
       t.id AS "track_id",
       t.position AS "position",
       t.name AS "track_title",
       -- Length in MB data is in milliseconds
       ROUND(t.length / 1000)::INTEGER AS "length",
       rel.id AS "release_id",
       rel.release_group AS "release_group_id",
       rel.name AS "album_title",
       rel.comment AS "release_comment",
       CASE
           WHEN rc.date_year IS NULL THEN
               ARRAY[ ruc.date_year, ruc.date_month, ruc.date_day ]
           ELSE
               ARRAY[ rc.date_year, rc.date_month, rc.date_day ]
       END AS "release_date",
       ( SELECT ARRAY[ od.date_year, od.date_month, od.date_day ] FROM (
           SELECT original_date.date_year, original_date.date_month, original_date.date_day
           FROM (
               SELECT rc2.date_year, rc2.date_month, rc2.date_day,
                      ARRAY_TO_STRING(
                          ARRAY[
                              rc2.date_year::TEXT,
                              LPAD(rc2.date_month::TEXT, 2, '0'),
                              LPAD(rc2.date_day::TEXT, 2, '0')
                          ], '-', 'N'
                      ) AS ordering
                 FROM musicbrainz.release_group AS rg2
                 JOIN musicbrainz.release AS r2 ON rg2.id = r2.release_group
                 JOIN musicbrainz.release_country AS rc2 ON r2.id = rc2.release
                WHERE rg2.id = ( SELECT release_group FROM musicbrainz.release AS r3 WHERE r3.id = rel.id )
               UNION
               SELECT ruc2.date_year, ruc2.date_month, ruc2.date_day,
                      ARRAY_TO_STRING(
                          ARRAY[
                              ruc2.date_year::TEXT,
                              LPAD(ruc2.date_month::TEXT, 2, '0'),
                              LPAD(ruc2.date_day::TEXT, 2, '0')
                          ], '-', 'N'
                      ) AS ordering
                 FROM musicbrainz.release_group AS rg2
                 JOIN musicbrainz.release AS r2 ON rg2.id = r2.release_group
                 JOIN musicbrainz.release_unknown_country AS ruc2 ON r2.id = ruc2.release
                WHERE rg2.id = ( SELECT release_group FROM musicbrainz.release AS r3 WHERE r3.id = rel.id )
           ) AS original_date
           ORDER BY ordering
           LIMIT 1
       ) as od ) AS "original_release_date",
       a.id AS "artist_id",
       a.name AS "artist",
       rec.id AS "recording_id",
       rec.gid AS "recording_gid"
  FROM musicbrainz.track AS t
  JOIN musicbrainz.medium AS m ON t.medium = m.id
  JOIN musicbrainz.release AS rel ON m.release = rel.id
  LEFT OUTER JOIN musicbrainz.release_alias AS ra ON rel.id = ra.release
  LEFT OUTER JOIN musicbrainz.release_country AS rc ON rel.id = rc.release
  LEFT OUTER JOIN musicbrainz.release_unknown_country AS ruc ON rel.id = ruc.release
  JOIN musicbrainz.release_group AS rg ON rel.release_group = rg.id
  JOIN musicbrainz.artist_credit AS ac ON rel.artist_credit = ac.id
  JOIN musicbrainz.artist_credit_name AS acn ON ac.id = acn.artist_credit
  JOIN musicbrainz.artist AS a ON acn.artist = a.id
  LEFT OUTER JOIN musicbrainz.artist_alias AS aa ON a.id = aa.artist
  JOIN musicbrainz.recording AS rec ON t.recording = rec.id
"#;

impl DB {
    pub async fn new(db_uri: &str) -> DBResult<Self> {
        Ok(Self {
            pool: PgPoolOptions::new()
                .max_connections(10)
                .connect(db_uri)
                .await?,
        })
    }

    pub async fn get_or_insert_user<E: AsRef<str>>(&self, email: E) -> DBResult<User> {
        let user = sqlx::query_as!(
            User,
            r#"
                SELECT user_id,
                       email AS "email!: String",
                       date_format,
                       preferred_name_order AS "preferred_name_order: _"
                  FROM crumb."user"
                 WHERE email = $1
            "#,
            email.as_ref(),
        )
        .fetch_optional(&self.pool)
        .await?;
        if let Some(user) = user {
            return Ok(user);
        }

        sqlx::query_as!(
            User,
            r#"
                INSERT INTO crumb."user" (email)
                VALUES ($1::TEXT::crumb.email)
                RETURNING
                    user_id,
                    email AS "email!: String",
                    date_format,
                    preferred_name_order AS "preferred_name_order: _"
            "#,
            email.as_ref(),
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.into())
    }

    pub async fn best_matches_for_tracks(
        &self,
        tracks: &[&TrackInfo],
    ) -> DBResult<HashMap<i32, Vec<TrackMatch>>> {
        let mut album: HashMap<i32, Vec<TrackMatch>> = HashMap::new();
        for &track in tracks {
            debug!(
                "Matching title = {:?} position = {:?} album = {:?} artist = {:?}",
                track.title, track.position, track.album, track.artist,
            );
            // CREATE INDEX track_lower_unaccent_name_position ON musicbrainz.track (lower(musicbrainz.musicbrainz_unaccent(name)), position);
            //
            // This index is needed to make the query performant. Without the
            // index it's quite slow.
            //
            // Optimizing this was very weird. If we use the simple OR clause
            // for both release & artist, it ends up doing table
            // scans. Similarly, if we use the subselect for both, it's also
            // slower. But if we use the simple OR clause for release and the
            // subselect for artist, it's great. If we _swap_ those around,
            // it's slow again! WTF, query planner?!
            let select = format!(
                "{}{}",
                TRACK_MATCH_SELECT,
                r#"
                    WHERE LOWER(musicbrainz.musicbrainz_unaccent(t.name)) = LOWER(musicbrainz.musicbrainz_unaccent($1))
                      AND t.position = $2
                      AND (
                            LOWER(musicbrainz.musicbrainz_unaccent(rel.name)) = LOWER(musicbrainz.musicbrainz_unaccent($3))
                            OR LOWER(musicbrainz.musicbrainz_unaccent(ra.name)) = LOWER(musicbrainz.musicbrainz_unaccent($3))
                          )
                      AND (
                            LOWER(musicbrainz.musicbrainz_unaccent(a.name)) = LOWER(musicbrainz.musicbrainz_unaccent($4))
                            OR LOWER(musicbrainz.musicbrainz_unaccent(a.sort_name)) = LOWER(musicbrainz.musicbrainz_unaccent($4))
                            OR a.id IN (
                                SELECT artist
                                  FROM musicbrainz.artist_alias
                                  WHERE LOWER(musicbrainz.musicbrainz_unaccent(name)) = LOWER(musicbrainz.musicbrainz_unaccent($4))
                            )
                          )
                "#,
            );
            let matches = sqlx::query_as::<_, TrackMatch>(&select)
                .bind(&track.title)
                .bind(track.position)
                .bind(&track.album)
                .bind(&track.artist)
                .fetch_all(&self.pool)
                .await?;
            album.insert(track.position, matches);
        }
        Ok(album)
    }

    pub async fn match_track_gids(
        &self,
        track_ids: &[Uuid],
    ) -> DBResult<HashMap<i32, Vec<TrackMatch>>> {
        let mut album: HashMap<i32, Vec<TrackMatch>> = HashMap::new();
        debug!("Matching track ids: {:?}", track_ids);
        let select = format!("{}{}", TRACK_MATCH_SELECT, "WHERE t.gid = ANY($1)");
        let matches = sqlx::query_as::<_, TrackMatch>(&select)
            .bind(track_ids)
            .fetch_all(&self.pool)
            .await?;
        for m in matches {
            album.insert(m.position, vec![m]);
        }
        Ok(album)
    }

    pub async fn insert_user_tracks(
        &self,
        user_id: &Uuid,
        matches_with_hashes: &[(&TrackMatch, &str)],
    ) -> DBResult<()> {
        let artist_ids = matches_with_hashes
            .iter()
            .map(|m| m.0.artist_id)
            .unique()
            .collect::<Vec<_>>();
        if artist_ids.len() > 1 {
            return Err(DBError::CannotInsertUserTracksForMultipleArtists);
        }
        let release_ids = matches_with_hashes
            .iter()
            .map(|m| m.0.release_id)
            .unique()
            .collect::<Vec<_>>();
        if release_ids.len() > 1 {
            return Err(DBError::CannotInsertUserTracksForMultipleReleases);
        }

        let mut tx = self.pool.begin().await?;
        let artist = self.insert_artist(&mut tx, artist_ids[0]).await?;
        let release = self
            .insert_release(
                &mut tx,
                &artist.artist_id,
                &matches_with_hashes.iter().map(|m| m.0).collect::<Vec<_>>(),
            )
            .await?;
        for m in matches_with_hashes {
            self.insert_user_track(
                &mut tx,
                &user_id,
                &artist.artist_id,
                &release.release_id,
                m.0,
                m.1,
            )
            .await?;
        }
        tx.commit().await?;

        Ok(())
    }

    async fn insert_artist(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        mb_artist_id: i32,
    ) -> DBResult<Artist> {
        let possible_names = self.artist_names_and_aliases(tx, mb_artist_id).await?;
        let names = names_and_aliases(&possible_names);

        debug!("Inserting artist with MB artist id {}:", mb_artist_id);
        debug!(
            "  Name = {} ({})",
            names.name,
            names.sortable_name.unwrap_or("<no sortable name>")
        );
        debug!(
            "  Transcripted name = {} ({})",
            names.transcripted_name.unwrap_or("<no transcripted name>"),
            names
                .transcripted_sortable_name
                .unwrap_or("<no transcripted sortable name>"),
        );
        debug!(
            "  Translated name = {} ({})",
            names.translated_name.unwrap_or("<no translated name>"),
            names
                .translated_sortable_name
                .unwrap_or("<no translated sortable name>"),
        );

        let artist = sqlx::query_as!(
            Artist,
            r#"
                WITH ins AS (
                    INSERT INTO crumb.artist
                        (
                            musicbrainz_artist_id, name, sortable_name,
                            transcripted_name, transcripted_sortable_name,
                            translated_name, translated_sortable_name
                         )
                    VALUES
                        (
                            $1, $2::TEXT::crumb.non_empty_citext, $3::TEXT::crumb.non_empty_citext,
                            $4::TEXT::crumb.non_empty_citext, $5::TEXT::crumb.non_empty_citext,
                            $6::TEXT::crumb.non_empty_citext, $7::TEXT::crumb.non_empty_citext
                        )
                    ON CONFLICT (musicbrainz_artist_id) DO NOTHING
                    RETURNING *
                )
                SELECT artist_id AS "artist_id!",
                       musicbrainz_artist_id,
                       name::TEXT AS "name!",
                       sortable_name::TEXT AS "sortable_name!"
                  FROM crumb.artist
                 WHERE musicbrainz_artist_id = $1
                UNION
                SELECT artist_id AS "artist_id!",
                       musicbrainz_artist_id,
                       name::TEXT AS "name!",
                       sortable_name::TEXT AS "sortable_name!"
                  FROM ins
            "#,
            mb_artist_id,
            names.name,
            names.sortable_name.unwrap_or(names.name),
            names.transcripted_name,
            names.transcripted_sortable_name,
            names.translated_name,
            names.translated_sortable_name,
        )
        .fetch_one(&mut *tx)
        .await?;

        for alias in names.search_hint_aliases {
            sqlx::query!(
                r#"
                    INSERT INTO crumb.artist_search_hint
                        ( artist_id, hint )
                    VALUES
                        ( $1, $2::TEXT::crumb.non_empty_citext )
                    ON CONFLICT ( artist_id, hint ) DO NOTHING
                "#,
                artist.artist_id,
                alias,
            )
            .execute(&mut *tx)
            .await?;
        }

        Ok(artist)
    }

    async fn artist_names_and_aliases(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        mb_artist_id: i32,
    ) -> DBResult<Vec<Name>> {
        let mut mb_artist_names: Vec<Name> = vec![];
        mb_artist_names.push(
            sqlx::query_as!(
                MBName,
                r#"
                    SELECT name,
                           sort_name AS "sort_name: _",
                           'primary' AS "name_type!",
                           NULL as "locale"
                      FROM musicbrainz.artist
                     WHERE id = $1
                "#,
                mb_artist_id,
            )
            .fetch_one(&mut *tx)
            .await?
            .into(),
        );
        mb_artist_names.append(
            &mut sqlx::query_as!(
                MBName,
                r#"
                    SELECT aa.name,
                           aa.sort_name AS "sort_name: _",
                           CASE
                               WHEN aa.primary_for_locale THEN
                                    'primary'
                               WHEN aat.name = 'Artist name' THEN
                                   'alias'
                               WHEN aat.name = 'Search hint' THEN
                                   'search'
                               -- This happens if the alias has no type
                               ELSE
                                   'alias'
                           END AS "name_type!",
                           aa.locale
                      FROM musicbrainz.artist_alias AS aa
                      LEFT OUTER JOIN musicbrainz.artist_alias_type AS aat ON aa.type = aat.id
                     WHERE aa.artist = $1
                       AND aat.name != 'Legal name'
                "#,
                mb_artist_id,
            )
            .fetch_all(&mut *tx)
            .await?
            .into_iter()
            .map(|n| n.into())
            .collect(),
        );
        mb_artist_names.append(
            &mut sqlx::query_as!(
                MBName,
                r#"
                    SELECT acn.name AS "name!",
                           NULL AS "sort_name",
                           'alias' AS "name_type!",
                           NULL AS "locale"
                      FROM musicbrainz.artist AS a
                      JOIN musicbrainz.artist_credit_name AS acn ON a.id = acn.artist
                     WHERE a.id = $1
                "#,
                mb_artist_id,
            )
            .fetch_all(&mut *tx)
            .await?
            .into_iter()
            .map(|n| n.into())
            .collect(),
        );

        // If this is an artist with a non-latin name and a transcription or
        // translation as the sortable name, then it's possible that the only
        // occurrence of a particular alias is that sortable name, so we need
        // to include that too. The sortable name is always empty for
        // releases.
        let unique_sort_names: HashSet<&str> = mb_artist_names
            .iter()
            .filter_map(|m| m.sort_name.as_deref())
            .collect();
        let unique_names: HashSet<&str> = mb_artist_names.iter().map(|m| m.name.as_str()).collect();
        let mut diff = unique_sort_names
            .difference(&unique_names)
            .map(|&n| {
                let (name, sort_name) = maybe_uncomma_name(n);
                Name {
                    name,
                    sort_name,
                    name_type: NameOrAliasType::AliasName,
                    locale: None,
                }
            })
            .collect();
        mb_artist_names.append(&mut diff);

        Ok(mb_artist_names)
    }

    async fn insert_release(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        primary_artist_id: &Uuid,
        matches: &[&TrackMatch],
    ) -> DBResult<Release> {
        let possible_names = self
            .release_names_and_aliases_with_siblings(tx, matches[0].release_id, matches)
            .await?;
        let names = names_and_aliases(&possible_names);
        debug!(
            "Inserting release with MB release id {}:",
            matches[0].release_id
        );
        debug!("  Name = {}", names.name,);
        debug!(
            "  Transcripted name = {}",
            names.transcripted_name.unwrap_or("<no transcripted name>"),
        );
        debug!(
            "  Translated name = {}",
            names.translated_name.unwrap_or("<no translated name>"),
        );

        let release = sqlx::query_as!(
            Release,
            r#"
                WITH ins AS (
                    INSERT INTO crumb.release
                        (
                             musicbrainz_release_id, primary_artist_id, title,
                             transcripted_title, translated_title,
                             comment,
                             release_year, release_month, release_day,
                             original_year, original_month, original_day
                        )
                    VALUES
                        (
                             $1, $2, $3::TEXT::crumb.non_empty_citext,
                             $4::TEXT::crumb.non_empty_citext, $5::TEXT::crumb.non_empty_citext,
                             $6::TEXT::crumb.non_empty_citext,
                             $7, $8, $9,
                             $10, $11, $12
                        )
                    ON CONFLICT (musicbrainz_release_id) DO NOTHING
                    RETURNING *
                )
                SELECT release_id AS "release_id!",
                       musicbrainz_release_id,
                       title::TEXT AS "title!",
                       release_year,
                       release_month,
                       release_day,
                       original_year,
                       original_month,
                       original_day
                  FROM crumb.release
                 WHERE musicbrainz_release_id = $1
                UNION
                SELECT release_id AS "release_id!",
                       musicbrainz_release_id,
                       title::TEXT AS "title!",
                       release_year,
                       release_month,
                       release_day,
                       original_year,
                       original_month,
                       original_day
                  FROM ins
            "#,
            matches[0].release_id,
            primary_artist_id,
            names.name,
            names.transcripted_name,
            names.translated_name,
            if matches[0].release_comment.is_empty() {
                None
            } else {
                Some(&matches[0].release_comment)
            },
            matches[0].release_date.get(0) as _,
            matches[0].release_date.get(1) as _,
            matches[0].release_date.get(2) as _,
            matches[0]
                .original_release_date
                .as_ref()
                .and_then(|ord| ord.get(0)) as _,
            matches[0]
                .original_release_date
                .as_ref()
                .and_then(|ord| ord.get(1)) as _,
            matches[0]
                .original_release_date
                .as_ref()
                .and_then(|ord| ord.get(2)) as _,
        )
        .fetch_one(&mut *tx)
        .await?;

        for alias in names.search_hint_aliases {
            sqlx::query!(
                r#"
                    INSERT INTO crumb.release_search_hint
                        ( release_id, hint )
                    VALUES
                        ( $1, $2::TEXT::crumb.non_empty_citext )
                    ON CONFLICT ( release_id, hint ) DO NOTHING
                "#,
                release.release_id,
                alias,
            )
            .execute(&mut *tx)
            .await?;
        }

        Ok(release)
    }

    async fn release_names_and_aliases_with_siblings(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        mb_release_id: i32,
        matches: &[&TrackMatch],
    ) -> DBResult<Vec<Name>> {
        let mut mb_release_names = self.release_names_and_aliases(tx, mb_release_id).await?;
        // We only want to look at siblings to find Latin-script
        // transcriptions or translation of non-Latin names. Otherwise we find
        // all sorts of other stuff, like "anniversary re-releases" and such.
        if mb_release_names
            .iter()
            .find(|&n| matches!(n.name_type, NameOrAliasType::PrimaryName) && !is_latin(&n.name))
            .is_some()
        {
            mb_release_names.append(
                &mut self
                    .sibling_release_names(tx, matches)
                    .await?
                    .into_iter()
                    .filter(|n| is_latin(&n.name))
                    .collect(),
            )
        }

        Ok(mb_release_names)
    }

    async fn release_names_and_aliases(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        mb_release_id: i32,
    ) -> DBResult<Vec<Name>> {
        let mut mb_release_names: Vec<Name> = vec![];
        mb_release_names.push(
            sqlx::query_as!(
                MBName,
                r#"
                    SELECT name,
                           NULL AS "sort_name: _",
                           'primary' AS "name_type!",
                           NULL AS "locale"
                      FROM musicbrainz.release
                     WHERE id = $1
                "#,
                mb_release_id
            )
            .fetch_one(&mut *tx)
            .await?
            .into(),
        );
        mb_release_names.append(
            &mut sqlx::query_as!(
                MBName,
                r#"
                    SELECT ra.name,
                           ra.sort_name AS "sort_name: _",
                           CASE
                               WHEN ra.primary_for_locale THEN
                                    'primary'
                               WHEN rat.name = 'Release name' THEN
                                   'alias'
                               WHEN rat.name = 'Search hint' THEN
                                   'search'
                               -- This happens if the alias has no type
                               ELSE
                                   'alias'
                           END AS "name_type!",
                           ra.locale
                      FROM musicbrainz.release_alias AS ra
                      LEFT OUTER JOIN musicbrainz.release_alias_type AS rat ON ra.type = rat.id
                     WHERE ra.release = $1
                       AND rat.name != 'Legal name'
                "#,
                mb_release_id
            )
            .fetch_all(&mut *tx)
            .await?
            .into_iter()
            .map(|n| n.into())
            .collect(),
        );
        Ok(mb_release_names)
    }

    // This approach of getting the release IDs and fetch the names for each
    // release separately is definitely slower, but it's a lot simpler. We can
    // always refactor if it turns out to be a speed bump.
    async fn sibling_release_names(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        matches: &[&TrackMatch],
    ) -> DBResult<Vec<Name>> {
        let track_positions = matches
            .iter()
            .map(|m| m.position)
            .sorted()
            .collect::<Vec<_>>();
        let sibling_ids = sqlx::query!(
            r#"
                SELECT r.id
                  FROM musicbrainz.release AS r
                 WHERE release_group = (
                           SELECT release_group
                             FROM musicbrainz.release AS r2
                            WHERE r2.id = $1
                       )
                   AND r.id != $1
                   AND ARRAY(
                           SELECT t.position
                             FROM musicbrainz.track AS t
                             JOIN musicbrainz.medium AS m ON t.medium = m.id
                            WHERE m.release = r.id
                           ORDER BY t.position
                       ) = $2
            "#,
            matches[0].release_id,
            &track_positions,
        )
        .fetch_all(&mut *tx)
        .await?;

        let mut sibling_names: Vec<Name> = vec![];
        for id in sibling_ids {
            sibling_names.append(&mut self.release_names_and_aliases(tx, id.id).await?);
        }

        Ok(sibling_names)
    }

    async fn insert_user_track(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user_id: &Uuid,
        artist_id: &Uuid,
        release_id: &Uuid,
        m: &TrackMatch,
        hash: &str,
    ) -> DBResult<()> {
        sqlx::query!(
            r#"
                WITH ins_track AS (
                    INSERT INTO crumb.track
                        ( musicbrainz_track_id, primary_artist_id, title, length, content_hash )
                    VALUES
                        ( $1, $2, $3::TEXT::crumb.non_empty_citext, $4::int::crumb.positive_int, $5::TEXT::crumb.non_empty_citext )
                    ON CONFLICT (musicbrainz_track_id) DO NOTHING
                    RETURNING *
                ), new_track AS (
                    SELECT track_id
                      FROM crumb.track WHERE musicbrainz_track_id = $1
                    UNION
                    SELECT track_id
                      FROM ins_track
                ), ins_release_track AS (
                    INSERT INTO crumb.release_track
                        ( release_id, track_id, position )
                    VALUES ( $6, ( SELECT new_track.track_id FROM new_track ), $7::int::crumb.positive_int )
                    ON CONFLICT ( release_id, track_id ) DO NOTHING
                )
                INSERT INTO crumb.user_track
                    ( user_id, track_id )
                SELECT $8, new_track.track_id
                  FROM new_track
                ON CONFLICT ( user_id, track_id ) DO NOTHING

            "#,
            Some(m.track_id),
            artist_id,
            m.track_title,
            m.length,
            hash,
            release_id,
            m.position,
            user_id,
        )
        .execute(&mut *tx)
        .await?;

        Ok(())
    }
}

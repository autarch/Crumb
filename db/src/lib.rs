pub mod dbtypes;
mod names;

pub use crate::dbtypes::*;
use crate::names::*;
use itertools::Itertools;
use log::debug;
pub use sqlx::Error as SQLXError;
use sqlx::{
    postgres::{PgPool, PgPoolOptions, PgRow},
    Executor, Postgres, Row, Transaction,
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
       at.name AS "artist_type",
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
  LEFT OUTER JOIN musicbrainz.artist_type AS at ON a.type = at.id
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

    #[tracing::instrument(skip(self))]
    pub async fn get_or_insert_user<E: AsRef<str> + std::fmt::Debug>(
        &self,
        email: E,
    ) -> DBResult<User> {
        if let Some(user) = self.get_user(&email).await? {
            return Ok(user);
        }

        sqlx::query_as::<_, User>(
            r#"
                INSERT INTO crumb."user" (email)
                VALUES ($1)
                RETURNING
                    user_id,
                    email,
                    date_format,
                    preferred_name_order
            "#,
        )
        .bind(email.as_ref())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.into())
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_user<E: AsRef<str> + std::fmt::Debug>(
        &self,
        email: E,
    ) -> DBResult<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            r#"
                SELECT user_id,
                       email,
                       date_format,
                       preferred_name_order
                  FROM crumb."user"
                 WHERE email = $1
            "#,
        )
        .bind(email.as_ref())
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    #[tracing::instrument(skip(self))]
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

    #[tracing::instrument(skip(self))]
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

    #[tracing::instrument(skip(self))]
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
        let artist = self
            .insert_artist(&mut tx, matches_with_hashes[0].0)
            .await?;
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

    #[tracing::instrument(skip(self))]
    async fn insert_artist(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        m: &TrackMatch,
    ) -> DBResult<Artist> {
        let possible_names = self.artist_names_and_aliases(tx, m.artist_id).await?;
        let names = names_and_aliases(
            &possible_names,
            matches!(m.artist_type.as_deref(), Some("Person")),
        );

        debug!("Inserting artist with MB artist id {}:", m.artist_id);
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

        let artist = sqlx::query_as::<_, Artist>(
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
                            $1, $2, $3,
                            $4, $5,
                            $6, $7
                        )
                    ON CONFLICT (musicbrainz_artist_id) DO NOTHING
                    RETURNING *
                )
                SELECT artist_id,
                       musicbrainz_artist_id,
                       name,
                       sortable_name
                  FROM crumb.artist
                 WHERE musicbrainz_artist_id = $1
                UNION
                SELECT artist_id,
                       musicbrainz_artist_id,
                       name,
                       sortable_name
                  FROM ins
            "#,
        )
        .bind(m.artist_id)
        .bind(names.name)
        .bind(names.sortable_name.unwrap_or(names.name))
        .bind(names.transcripted_name)
        .bind(names.transcripted_sortable_name)
        .bind(names.translated_name)
        .bind(names.translated_sortable_name)
        .fetch_one(&mut *tx)
        .await?;

        for alias in names.search_hint_aliases {
            sqlx::query(
                r#"
                    INSERT INTO crumb.artist_search_hint
                        ( artist_id, hint )
                    VALUES
                        ( $1, $2 )
                    ON CONFLICT ( artist_id, hint ) DO NOTHING
                "#,
            )
            .bind(artist.artist_id)
            .bind(alias)
            .execute(&mut *tx)
            .await?;
        }

        Ok(artist)
    }

    #[tracing::instrument(skip(self))]
    async fn artist_names_and_aliases(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        mb_artist_id: i32,
    ) -> DBResult<Vec<Name>> {
        let mut mb_artist_names: Vec<Name> = vec![];
        mb_artist_names.push(
            sqlx::query_as::<_, MBName>(
                r#"
                    SELECT name,
                           sort_name,
                           'primary' AS "name_type",
                           NULL as "locale"
                      FROM musicbrainz.artist
                     WHERE id = $1
                "#,
            )
            .bind(mb_artist_id)
            .fetch_one(&mut *tx)
            .await?
            .into(),
        );
        mb_artist_names.append(
            &mut sqlx::query_as::<_, MBName>(
                r#"
                    SELECT aa.name,
                           aa.sort_name,
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
                           END AS "name_type",
                           aa.locale
                      FROM musicbrainz.artist_alias AS aa
                      LEFT OUTER JOIN musicbrainz.artist_alias_type AS aat ON aa.type = aat.id
                     WHERE aa.artist = $1
                       AND aat.name != 'Legal name'
                "#,
            )
            .bind(mb_artist_id)
            .fetch_all(&mut *tx)
            .await?
            .into_iter()
            .map(|n| n.into())
            .collect(),
        );
        mb_artist_names.append(
            &mut sqlx::query_as::<_, MBName>(
                r#"
                    SELECT acn.name,
                           NULL AS "sort_name",
                           'alias' AS "name_type",
                           NULL AS "locale"
                      FROM musicbrainz.artist AS a
                      JOIN musicbrainz.artist_credit_name AS acn ON a.id = acn.artist
                     WHERE a.id = $1
                "#,
            )
            .bind(mb_artist_id)
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

    #[tracing::instrument(skip(self))]
    async fn insert_release(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        primary_artist_id: &Uuid,
        matches: &[&TrackMatch],
    ) -> DBResult<Release> {
        let possible_names = self
            .release_names_and_aliases_with_siblings(tx, matches[0].release_id, matches)
            .await?;
        let names = names_and_aliases(&possible_names, false);
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

        let release = sqlx::query_as::<_, Release>(
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
                             $1, $2, $3,
                             $4, $5,
                             $6,
                             $7, $8, $9,
                             $10, $11, $12
                        )
                    ON CONFLICT (musicbrainz_release_id) DO NOTHING
                    RETURNING *
                )
                SELECT release_id,
                       musicbrainz_release_id,
                       title,
                       release_year,
                       release_month,
                       release_day,
                       original_year,
                       original_month,
                       original_day
                  FROM crumb.release
                 WHERE musicbrainz_release_id = $1
                UNION
                SELECT release_id,
                       musicbrainz_release_id,
                       title,
                       release_year,
                       release_month,
                       release_day,
                       original_year,
                       original_month,
                       original_day
                  FROM ins
            "#,
        )
        .bind(matches[0].release_id)
        .bind(primary_artist_id)
        .bind(names.name)
        .bind(names.transcripted_name)
        .bind(names.translated_name)
        .bind(if matches[0].release_comment.is_empty() {
            None
        } else {
            Some(&matches[0].release_comment)
        })
        .bind(matches[0].release_date.get(0))
        .bind(matches[0].release_date.get(1))
        .bind(matches[0].release_date.get(2))
        .bind(
            matches[0]
                .original_release_date
                .as_ref()
                .and_then(|ord| ord.get(0)),
        )
        .bind(
            matches[0]
                .original_release_date
                .as_ref()
                .and_then(|ord| ord.get(1)),
        )
        .bind(
            matches[0]
                .original_release_date
                .as_ref()
                .and_then(|ord| ord.get(2)),
        )
        .fetch_one(&mut *tx)
        .await?;

        for alias in names.search_hint_aliases {
            sqlx::query(
                r#"
                    INSERT INTO crumb.release_search_hint
                        ( release_id, hint )
                    VALUES
                        ( $1, $2 )
                    ON CONFLICT ( release_id, hint ) DO NOTHING
                "#,
            )
            .bind(release.release_id)
            .bind(alias)
            .execute(&mut *tx)
            .await?;
        }

        Ok(release)
    }

    #[tracing::instrument(skip(self))]
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

    #[tracing::instrument(skip(self))]
    async fn release_names_and_aliases(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        mb_release_id: i32,
    ) -> DBResult<Vec<Name>> {
        let mut mb_release_names: Vec<Name> = vec![];
        mb_release_names.push(
            sqlx::query_as::<_, MBName>(
                r#"
                    SELECT name,
                           NULL AS "sort_name",
                           'primary' AS "name_type",
                           NULL AS "locale"
                      FROM musicbrainz.release
                     WHERE id = $1
                "#,
            )
            .bind(mb_release_id)
            .fetch_one(&mut *tx)
            .await?
            .into(),
        );
        mb_release_names.append(
            &mut sqlx::query_as::<_, MBName>(
                r#"
                    SELECT ra.name,
                           ra.sort_name,
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
                           END AS "name_type",
                           ra.locale
                      FROM musicbrainz.release_alias AS ra
                      LEFT OUTER JOIN musicbrainz.release_alias_type AS rat ON ra.type = rat.id
                     WHERE ra.release = $1
                       AND rat.name != 'Legal name'
                "#,
            )
            .bind(mb_release_id)
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
        let sibling_ids = sqlx::query(
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
        )
        .bind(matches[0].release_id)
        .bind(&track_positions)
        .map(|row: PgRow| row.get(0))
        .fetch_all(&mut *tx)
        .await?;

        let mut sibling_names: Vec<Name> = vec![];
        for id in sibling_ids {
            sibling_names.append(&mut self.release_names_and_aliases(tx, id).await?);
        }

        Ok(sibling_names)
    }

    #[tracing::instrument(skip(self))]
    async fn insert_user_track(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user_id: &Uuid,
        artist_id: &Uuid,
        release_id: &Uuid,
        m: &TrackMatch,
        hash: &str,
    ) -> DBResult<()> {
        sqlx::query(
            r#"
                WITH ins_track AS (
                    INSERT INTO crumb.track
                        ( musicbrainz_track_id, primary_artist_id, title, length, content_hash )
                    VALUES
                        ( $1, $2, $3, $4::int::crumb.positive_int, $5 )
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
        )
        .bind(Some(m.track_id))
        .bind(artist_id)
        .bind(&m.track_title)
        .bind(m.length)
        .bind(hash)
        .bind(release_id)
        .bind(m.position)
        .bind(user_id)
        .execute(&mut *tx)
        .await?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn artists_for_user(&self, user: &User) -> DBResult<Vec<ArtistListItem>> {
        let select = format!(
            r#"
                SELECT a.artist_id,
                       COALESCE({display_order:}) AS display_name,
                       a.name,
                       a.sortable_name,
                       a.transcripted_name,
                       a.transcripted_sortable_name,
                       a.translated_name,
                       a.translated_sortable_name,
                       COUNT(DISTINCT r.release_id) AS release_count,
                       COUNT(DISTINCT t.track_id) AS track_count,
                       crumb.best_cover_image_for_artist(a.artist_id, $1) AS release_cover_uri
                  FROM crumb.artist AS a
                  JOIN crumb.release AS r ON a.artist_id = r.primary_artist_id
                  JOIN crumb.release_track AS rt USING (release_id)
                  JOIN crumb.track AS t USING (track_id)
                  JOIN crumb.user_track AS ut USING (track_id)
                 WHERE ut.user_id = $1
                GROUP BY a.artist_id
                ORDER BY COALESCE({sort_order:}) ASC
            "#,
            display_order = user.display_order("a", SortableThing::Artist),
            sort_order = user.sort_order("a", SortableThing::Artist),
        );
        sqlx::query_as::<_, ArtistListItem>(&select)
            .bind(&user.user_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.into())
    }

    #[tracing::instrument(skip(self))]
    pub async fn artist_for_user(&self, user: &User, artist_id: &Uuid) -> DBResult<ArtistItem> {
        let select = format!(
            r#"
                SELECT a.artist_id,
                       COALESCE({display_order:}) AS display_name,
                       a.name,
                       a.sortable_name,
                       a.transcripted_name,
                       a.transcripted_sortable_name,
                       a.translated_name,
                       a.translated_sortable_name,
                       COUNT(DISTINCT r.release_id) AS release_count,
                       COUNT(DISTINCT t.track_id) AS track_count,
                       crumb.best_cover_image_for_artist(a.artist_id, $1) AS release_cover_uri
                  FROM crumb.artist AS a
                  JOIN crumb.release AS r ON a.artist_id = r.primary_artist_id
                  JOIN crumb.release_track AS rt USING (release_id)
                  JOIN crumb.track AS t USING (track_id)
                  JOIN crumb.user_track AS ut USING (track_id)
                 WHERE ut.user_id = $1
                   AND a.artist_id = $2
                GROUP BY a.artist_id
                ORDER BY COALESCE({sort_order:}) ASC
            "#,
            display_order = user.display_order("a", SortableThing::Artist),
            sort_order = user.sort_order("a", SortableThing::Artist),
        );
        let core = sqlx::query_as::<_, ArtistListItem>(&select)
            .bind(&user.user_id)
            .bind(artist_id)
            .fetch_one(&self.pool)
            .await?;
        let releases = self.releases_for_user_by_artist_id(user, artist_id).await?;
        Ok(ArtistItem { core, releases })
    }

    pub async fn releases_for_user_by_artist_id(
        &self,
        user: &User,
        artist_id: &Uuid,
    ) -> DBResult<Vec<ReleaseListItem>> {
        let select = format!(
            r#"
                SELECT r.release_id,
                       r.primary_artist_id,
                       COALESCE({display_order:}) AS display_title,
                       r.title,
                       r.transcripted_title,
                       r.translated_title,
                       r.comment,
                       COUNT(DISTINCT t.track_id) AS track_count,
                       r.release_year,
                       r.release_month,
                       r.release_day,
                       r.original_year,
                       r.original_month,
                       r.original_day,
                       crumb.release_date_for_release(r) release_date,
                       crumb.best_cover_image_for_release(mr.id, mr.release_group) AS release_cover_uri
                  FROM crumb.release AS r
                  JOIN crumb.release_track AS rt USING (release_id)
                  JOIN crumb.track AS t USING (track_id)
                  JOIN crumb.user_track AS ut USING (track_id)
                  JOIN musicbrainz.release AS mr ON r.musicbrainz_release_id = mr.id
                 WHERE ut.user_id = $1
                   AND r.primary_artist_id = $2
                GROUP BY r.release_id, mr.id
                ORDER BY release_date ASC
            "#,
            display_order = user.display_order("r", SortableThing::Release),
        );
        sqlx::query_as::<_, ReleaseListItem>(&select)
            .bind(&user.user_id)
            .bind(artist_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.into())
    }

    #[tracing::instrument(skip(self))]
    pub async fn release_for_user(&self, user: &User, release_id: &Uuid) -> DBResult<ReleaseItem> {
        let select = format!(
            r#"
                SELECT r.release_id,
                       r.primary_artist_id,
                       COALESCE({display_order:}) AS display_title,
                       r.title,
                       r.transcripted_title,
                       r.translated_title,
                       r.comment,
                       COUNT(DISTINCT t.track_id) AS track_count,
                       r.release_year,
                       r.release_month,
                       r.release_day,
                       r.original_year,
                       r.original_month,
                       r.original_day,
                       crumb.release_date_for_release(r) release_date,
                       crumb.best_cover_image_for_release(mr.id, mr.release_group) AS release_cover_uri
                  FROM crumb.release AS r
                  JOIN crumb.release_track AS rt USING (release_id)
                  JOIN crumb.track AS t USING (track_id)
                  JOIN crumb.user_track AS ut USING (track_id)
                  JOIN musicbrainz.release AS mr ON r.musicbrainz_release_id = mr.id
                 WHERE ut.user_id = $1
                   AND r.release_id = $2
                GROUP BY r.release_id, mr.id
                ORDER BY release_date ASC
            "#,
            display_order = user.display_order("r", SortableThing::Release),
        );
        let core = sqlx::query_as::<_, ReleaseListItem>(&select)
            .bind(&user.user_id)
            .bind(release_id)
            .fetch_one(&self.pool)
            .await?;
        let tracks = self.tracks_for_user_by_release_id(user, release_id).await?;
        let artist = self.artist_for_user(user, &core.primary_artist_id).await?;
        Ok(ReleaseItem {
            core,
            artist: artist.core,
            tracks,
        })
    }

    #[tracing::instrument(skip(self))]
    pub async fn tracks_for_user_by_release_id(
        &self,
        user: &User,
        release_id: &Uuid,
    ) -> DBResult<Vec<ReleaseTrack>> {
        let select = format!(
            r#"
                SELECT t.track_id,
                       t.primary_artist_id,
                       COALESCE({display_order:}) AS display_title,
                       t.title,
                       t.transcripted_title,
                       t.translated_title,
                       t.length,
                       t.content_hash,
                       rt.release_id,
                       rt.position
                  FROM crumb.release AS r
                  JOIN crumb.release_track AS rt USING (release_id)
                  JOIN crumb.track AS t USING (track_id)
                  JOIN crumb.user_track AS ut USING (track_id)
                  JOIN musicbrainz.release AS mr ON r.musicbrainz_release_id = mr.id
                 WHERE ut.user_id = $1
                   AND r.release_id = $2
                ORDER BY rt.position ASC
            "#,
            display_order = user.display_order("t", SortableThing::Track),
        );
        sqlx::query_as::<_, ReleaseTrack>(&select)
            .bind(&user.user_id)
            .bind(release_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.into())
    }

    //    #[tracing::instrument(skip(self))]
    pub async fn queue_for_user(&self, user: &User, client_id: &Uuid) -> DBResult<Vec<QueueItem>> {
        let select = format!(
            r#"
                SELECT t.track_id,
                       t.primary_artist_id,
                       COALESCE({track_display_order:}) AS display_title,
                       t.title,
                       t.transcripted_title,
                       t.translated_title,
                       t.length,
                       t.content_hash,
                       rt.release_id,
                       rt.position,
                       r.release_id,
                       COALESCE({release_display_order:}) AS release_display_title,
                       crumb.best_cover_image_for_release(mr.id, mr.release_group) AS release_cover_uri,
                       a.artist_id,
                       COALESCE({artist_display_order:}) AS artist_display_name,
                       uq.position AS queue_position,
                       uq.is_current
                  FROM crumb.user_queue AS uq
                  JOIN crumb.track AS t USING (track_id)
                  JOIN crumb.release_track AS rt USING (track_id)
                  JOIN crumb.release AS r USING (release_id)
                  JOIN crumb.artist AS a ON r.primary_artist_id = a.artist_id
                  JOIN musicbrainz.release AS mr ON r.musicbrainz_release_id = mr.id
                 WHERE uq.user_id = $1
                   AND uq.client_id = $2
                ORDER BY uq.position ASC
            "#,
            track_display_order = user.display_order("t", SortableThing::Track),
            release_display_order = user.display_order("r", SortableThing::Release),
            artist_display_order = user.display_order("a", SortableThing::Artist),
        );
        // XXX - is there an easier way to do this without the gross map? I
        // can't use have the query return `ROW( ... ), uq.position` and then
        // query_as::<(ReleaseTrack, ReleaseListItem, i32)>. Nor can I make a tuple struct and
        // use that with that query.
        Ok(sqlx::query(&select)
            .bind(&user.user_id)
            .bind(client_id)
            .fetch_all(&self.pool)
            .await?
            .iter()
            .map(|qi| QueueItem {
                release_track: ReleaseTrack {
                    track_id: qi.get("track_id"),
                    primary_artist_id: qi.get("primary_artist_id"),
                    display_title: qi.get("display_title"),
                    title: qi.get("title"),
                    transcripted_title: qi.get("transcripted_title"),
                    translated_title: qi.get("translated_title"),
                    length: qi.get("length"),
                    content_hash: qi.get("content_hash"),
                    release_id: qi.get("release_id"),
                    position: qi.get("position"),
                },
                release_id: qi.get("release_id"),
                release_display_title: qi.get("release_display_title"),
                release_cover_uri: qi.get("release_cover_uri"),
                artist_id: qi.get("artist_id"),
                artist_display_name: qi.get("artist_display_name"),
                queue_position: qi.get("queue_position"),
                is_current: qi.get("is_current"),
            })
            .collect::<Vec<_>>())
    }

    #[tracing::instrument(skip(self))]
    pub async fn add_to_queue_for_user(
        &self,
        user: &User,
        client_id: &Uuid,
        track_ids: &[Uuid],
    ) -> DBResult<Vec<QueueItem>> {
        if track_ids.is_empty() {
            return Ok(vec![]);
        }

        let mut tx = self.pool.begin().await?;
        let create = r#"
            CREATE TEMPORARY TABLE tracks_to_queue (
                track_id  UUID  NOT NULL,
                position  INT  NOT NULL
            ) ON COMMIT DROP
        "#;
        tx.execute(create).await?;

        let values = track_ids
            .iter()
            .enumerate()
            .map(|(p, _)| format!("(${}, ${})", (p * 2) + 1, (p * 2) + 2))
            .collect::<Vec<_>>();
        let select_into_tracks_to_queue = format!(
            r#"
                INSERT INTO tracks_to_queue
                    (track_id, position)
                VALUES
                    {}
            "#,
            values.join(",\n    ")
        );
        let mut query = sqlx::query(&select_into_tracks_to_queue);
        for (p, id) in track_ids.iter().enumerate() {
            // We need to add one to p because when we do our insert later, we
            // add p to the max position in the current queue. If p is 0 then
            // we end up trying to insert a duplicate position into the queue.
            query = query.bind(id).bind((p + 1) as i32);
        }
        query.execute(&mut tx).await?;

        let insert = r#"
            WITH max_position AS (
                SELECT COALESCE( ROUND( MAX(position) ), 0 ) AS max_p
                  FROM crumb.user_queue
                 WHERE user_id = $1
            )
            INSERT INTO crumb.user_queue
                ( user_id, client_id, track_id, position )
            SELECT $1, $2, track_id, position + max_p
              FROM tracks_to_queue, max_position
        "#;
        sqlx::query(&insert)
            .bind(&user.user_id)
            .bind(client_id)
            .execute(&mut tx)
            .await?;

        let update = r#"
            WITH current_position AS (
                SELECT position AS current_position
                  FROM crumb.user_queue
                 WHERE user_id = $1
                   AND client_id = $2
                   AND is_current
            ), min_position AS (
                SELECT MIN(position) AS min_position
                  FROM crumb.user_queue
                 WHERE user_id = $1
                   AND client_id = $2
                GROUP BY ( user_id, client_id )
            )
            UPDATE crumb.user_queue
               SET is_current = true
             WHERE position = (
                       SELECT MAX(position)
                         FROM (
                                  SELECT current_position AS position
                                    FROM current_position
                                   UNION
                                  SELECT min_position AS position
                                    FROM min_position
                              ) AS p
                   )
        "#;
        sqlx::query(&update)
            .bind(&user.user_id)
            .bind(client_id)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;

        Ok(self.queue_for_user(user, client_id).await?)
    }

    #[tracing::instrument(skip(self))]
    pub async fn remove_from_queue_for_user(
        &self,
        user: &User,
        client_id: &Uuid,
        positions: &[String],
    ) -> DBResult<Vec<QueueItem>> {
        if positions.is_empty() {
            return Ok(vec![]);
        }

        let delete = r#"
            DELETE FROM crumb.user_queue
             WHERE user_id = $1
               AND client_id = $2
               AND position = ANY($3)
        "#;
        sqlx::query(&delete)
            .bind(&user.user_id)
            .bind(client_id)
            .bind(positions)
            .execute(&self.pool)
            .await?;

        Ok(self.queue_for_user(user, client_id).await?)
    }

    #[tracing::instrument(skip(self))]
    pub async fn move_queue_forward_for_user(
        &self,
        user: &User,
        client_id: &Uuid,
    ) -> DBResult<Vec<QueueItem>> {
        let update = r#"
            WITH current AS (
                SELECT position
                  FROM crumb.user_queue
                 WHERE user_id = $1
                   AND client_id = $2
                   AND is_current
            ), remove_current AS (
                UPDATE crumb.user_queue
                   SET is_current = false
                 WHERE user_id = $1
                   AND client_id = $2
                   AND position = ( SELECT position FROM current )
            )
            UPDATE crumb.user_queue
               SET is_current = true
             WHERE user_id = $1
               AND client_id = $2
               AND position = (
                       SELECT position
                         FROM crumb.user_queue
                        WHERE user_id = $1
                          AND client_id = $2
                          AND position > ( SELECT position FROM current )
                       ORDER BY position
                       LIMIT 1
                   )
        "#;
        sqlx::query(&update)
            .bind(&user.user_id)
            .bind(client_id)
            .execute(&self.pool)
            .await?;

        Ok(self.queue_for_user(user, client_id).await?)
    }

    #[tracing::instrument(skip(self))]
    pub async fn move_queue_backward_for_user(
        &self,
        user: &User,
        client_id: &Uuid,
    ) -> DBResult<Vec<QueueItem>> {
        let update = r#"
            WITH current AS (
                SELECT position
                  FROM crumb.user_queue
                 WHERE user_id = $1
                   AND client_id = $2
                   AND is_current
            ), remove_current AS (
                UPDATE crumb.user_queue
                   SET is_current = false
                 WHERE user_id = $1
                   AND client_id = $2
                   AND position = ( SELECT position FROM current )
            )
            UPDATE crumb.user_queue
               SET is_current = true
             WHERE user_id = $1
               AND client_id = $2
               AND position = (
                       SELECT position
                         FROM crumb.user_queue
                        WHERE user_id = $1
                          AND client_id = $2
                          AND position < ( SELECT position FROM current )
                       ORDER BY position DESC
                       LIMIT 1
                   )
        "#;
        sqlx::query(&update)
            .bind(&user.user_id)
            .bind(client_id)
            .execute(&self.pool)
            .await?;

        Ok(self.queue_for_user(user, client_id).await?)
    }

    #[tracing::instrument(skip(self))]
    pub async fn add_upvote_for_user(&self, user: &User, track_id: &Uuid) -> DBResult<()> {
        let update = r#"
            UPDATE crumb.user_track
               SET upvotes = upvotes + 1
             WHERE user_id = $1
               AND track_id = $2
        "#;
        sqlx::query(&update)
            .bind(&user.user_id)
            .bind(track_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn add_downvote_for_user(&self, user: &User, track_id: &Uuid) -> DBResult<()> {
        let update = r#"
            UPDATE crumb.user_track
               SET downvotes = downvotes + 1
             WHERE user_id = $1
               AND track_id = $2
        "#;
        sqlx::query(&update)
            .bind(&user.user_id)
            .bind(track_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

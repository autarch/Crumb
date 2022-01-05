-- Deploy crumb:functions-and-triggers to pg
-- requires: initial-schema

BEGIN;

CREATE OR REPLACE FUNCTION crumb.release_date_for_release(release crumb.release)
    RETURNS DATE
    IMMUTABLE
AS $$
BEGIN
    IF release.original_year IS NOT NULL THEN
        RETURN MAKE_DATE( release.original_year, COALESCE( release.original_month, 1 ), COALESCE( release.original_day, 1 ) );
    END IF;

    RETURN MAKE_DATE( COALESCE( release.release_year, 1800 ), COALESCE( release.release_month, 1 ), COALESCE( release.release_day, 1 ) );
END;
$$ LANGUAGE plpgsql;

-- We include the user_id so that we can filter this to only include albums
-- that the user has uploaded.
CREATE OR REPLACE FUNCTION crumb.best_cover_image_for_artist( artist_id UUID, for_user_id UUID )
    RETURNS TEXT
AS $$
DECLARE
    release RECORD;
    image_uri TEXT;
BEGIN
    -- This isn't the most efficient way to do this, because this could be a
    -- single query, but it's a lot simpler to reuse our
    -- best_cover_image_for_release function.
    FOR release IN
        SELECT mr.*, crumb.release_date_for_release(r) AS release_date
          FROM crumb.release AS r
          JOIN crumb.release_track AS rt USING (release_id)
          JOIN crumb.track AS t USING (track_id)
          JOIN crumb.user_track AS ut USING (track_id)
          JOIN musicbrainz.release AS mr ON r.musicbrainz_release_id = mr.id
         WHERE ut.user_id = for_user_id
           AND r.primary_artist_id = artist_id
        ORDER BY release_date DESC
    LOOP
        image_uri = crumb.best_cover_image_for_release(release.id, release.release_group);
        IF image_uri IS NOT NULL THEN
            RETURN image_uri;
        END IF;
    END LOOP;

    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION crumb.best_cover_image_for_release( mb_release_id INT, mb_release_group_id INT )
    RETURNS TEXT
AS $$
DECLARE
    mb_release_gid UUID;
    mb_cover_art_id BIGINT;
    mime_type TEXT;
BEGIN
    SELECT mr.gid,
           mca.id,
           mca.mime_type
      INTO mb_release_gid, mb_cover_art_id, mime_type
      FROM musicbrainz.release AS mr
      JOIN musicbrainz.cover_art AS mca ON mr.id = mca.release
      JOIN musicbrainz.cover_art_type AS mcat ON mca.id = mcat.id
     WHERE mr.id = mb_release_id
       AND mca.mime_type != 'application/pdf'
       AND mcat.type_id = 1
     LIMIT 1;

    -- If the release group has a selected front cover we use that.
    IF mb_release_gid IS NULL THEN
        SELECT mr.gid,
               mca.id,
               mca.mime_type
          INTO mb_release_gid, mb_cover_art_id, mime_type
          FROM musicbrainz.release AS mr
          JOIN musicbrainz.release_group_cover_art AS mrgca USING (release_group)
          JOIN musicbrainz.cover_art AS mca ON mrgca.release = mca.release
          JOIN musicbrainz.cover_art_type AS mcat ON mca.id = mcat.id
         WHERE mr.id = mb_release_id
           AND mca.mime_type != 'application/pdf'
           AND mcat.type_id = 1
         LIMIT 1;
    END IF;

    -- Otherwise just pick the cover for any release in the release group.
    IF mb_release_gid IS NULL THEN
        SELECT mr.gid,
               mca.id,
               mca.mime_type
          INTO mb_release_gid, mb_cover_art_id, mime_type
          FROM musicbrainz.release AS mr
          JOIN musicbrainz.cover_art AS mca ON mr.id = mca.release
          JOIN musicbrainz.cover_art_type AS mcat ON mca.id = mcat.id
         WHERE mca.release IN (
                   SELECT mr2.id
                     FROM musicbrainz.release AS mr2
                     JOIN musicbrainz.release_group AS mrg ON mr2.release_group = mrg.id
                    WHERE mrg.id = mb_release_group_id
               )
           AND mca.mime_type != 'application/pdf'
           AND mcat.type_id = 1
        ORDER BY mr.gid
         LIMIT 1;
    END IF;

    IF mb_release_gid IS NOT NULL THEN
        RETURN crumb.cover_image_url( mb_release_gid, mb_cover_art_id, mime_type );
    END IF;

    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION crumb.cover_image_url( mb_release_gid UUID, mb_cover_art_id BIGINT, mime_type TEXT )
    RETURNS TEXT
    IMMUTABLE
AS $$
DECLARE
    extension TEXT;
BEGIN
    IF mime_type = 'image/jpeg' THEN
        extension = 'jpg';
    ELSIF mime_type = 'image/png' THEN
        extension = 'jpg';
    ELSIF mime_type = 'image/gif' THEN
        extension = 'jpg';
    ELSE
        RETURN NULL;
    END IF;

    RETURN format(
        'http://s3.us.archive.org/mbid-%s/mbid-%s-%s.%s',
        mb_release_gid,
        mb_release_gid,
        mb_cover_art_id,
        extension
    );
END;
$$ LANGUAGE plpgsql;

COMMIT;

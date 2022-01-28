-- Deploy music-player:initial-schema to pg

\ir ../lib/if-exists.sql

BEGIN;

CREATE SCHEMA IF NOT EXISTS crumb;

SET search_path TO crumb;

CREATE EXTENSION IF NOT EXISTS citext;

DO $$
BEGIN

    IF NOT pg_temp.domain_exists('non_empty_citext') THEN
        CREATE DOMAIN non_empty_citext AS CITEXT
            CHECK ( VALUE != '' );
    END IF;

    IF NOT pg_temp.domain_exists('non_empty_text') THEN
        CREATE DOMAIN non_empty_text AS TEXT
            CHECK ( VALUE != '' );
    END IF;

    IF NOT pg_temp.domain_exists('email') THEN
        CREATE DOMAIN email AS CITEXT
            CHECK ( VALUE ~ E'^[a-zA-Z0-9.!#$%&''*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$' );

        COMMENT ON DOMAIN email IS
            'Obviously this isn''t a full email regex, but this is good'
            ' enough. To really test an email you have to send email to it'
            ' anyway. This regex will avoid obviously broken stuff.';
    END IF;

    IF NOT pg_temp.domain_exists('hash') THEN
        CREATE DOMAIN hash AS TEXT
            CHECK ( VALUE ~ E'^\\$[a-z0-9_\-]+\\$[0-9a-f]+$' );
    END IF;

    IF NOT pg_temp.domain_exists('year') THEN
        CREATE DOMAIN year AS SMALLINT
            CHECK ( VALUE >= 1877 AND VALUE <= EXTRACT(YEAR FROM CURRENT_DATE) + 1 );

        COMMENT ON DOMAIN year IS
            '1877 is the year of the first audio recording technology. Year +'
            ' 1 for something that someone imports on 12/31 that is a prerelease'
            ' of something from the next year.';
    END IF;

    IF NOT pg_temp.domain_exists('month') THEN
        CREATE DOMAIN month AS SMALLINT
            CHECK ( VALUE >= 1 AND VALUE <= 12 );
    END IF;

    IF NOT pg_temp.domain_exists('day') THEN
        CREATE DOMAIN day AS SMALLINT
            CHECK ( VALUE >= 1 AND VALUE <= 31 );
    END IF;

    IF NOT pg_temp.domain_exists('positive_int') THEN
        CREATE DOMAIN positive_int AS INT
            CHECK ( VALUE >= 1 );
    END IF;

    IF NOT pg_temp.domain_exists('name_type') THEN
        CREATE TYPE name_type AS ENUM ( 'original', 'transcripted', 'translated' );
    END IF;

    IF NOT pg_temp.domain_exists('sha_256') THEN
        CREATE DOMAIN sha_256 AS CITEXT
            CHECK ( VALUE ~ '^[0-9a-f]{64}$' );
    END IF;

END
$$;

CREATE TABLE IF NOT EXISTS "user" (
    user_id  UUID  PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    email  email  UNIQUE NOT NULL,
    date_format  non_empty_text  NOT NULL DEFAULT '%m-%d-%Y',
    preferred_name_order  name_type[]  NOT NULL  DEFAULT ARRAY['translated', 'transcripted', 'original']::name_type[],
    CONSTRAINT preferred_name_order_is_3 CHECK ( ARRAY_LENGTH(preferred_name_order, 1) == 3 )
);

CREATE TABLE IF NOT EXISTS artist (
    artist_id  UUID  PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    musicbrainz_artist_id  INTEGER  UNIQUE NULL
        REFERENCES musicbrainz.artist (id) ON UPDATE CASCADE ON DELETE CASCADE,
    name  non_empty_citext  NOT NULL,
    sortable_name  non_empty_citext  NOT NULL,
    transcripted_name  non_empty_citext  NULL,
    transcripted_sortable_name  non_empty_citext  NULL,
    translated_name  non_empty_citext  NULL,
    translated_sortable_name  non_empty_citext  NULL
);

CREATE INDEX IF NOT EXISTS artist_name ON artist (name);

CREATE TABLE IF NOT EXISTS artist_search_hint (
    artist_id  UUID  NOT NULL
        REFERENCES artist (artist_id) ON UPDATE CASCADE ON DELETE CASCADE,
    hint  non_empty_citext  NOT NULL,
    PRIMARY KEY ( artist_id, hint )
);

CREATE INDEX IF NOT EXISTS artist_search_hint_hint ON artist_search_hint (hint);

CREATE TABLE IF NOT EXISTS release (
    release_id  UUID  PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    musicbrainz_release_id  INTEGER  UNIQUE NULL
        REFERENCES musicbrainz.release (id) ON UPDATE CASCADE ON DELETE CASCADE,
    primary_artist_id  UUID  NOT NULL
        REFERENCES artist (artist_id) ON UPDATE CASCADE ON DELETE CASCADE,
    title  non_empty_citext  NOT NULL,
    transcripted_title  non_empty_citext  NULL,
    translated_title  non_empty_citext  NULL,
    comment non_empty_citext NULL,
    release_year  year  NULL,
    release_month  month  NULL,
    release_day  day  NULL,
    original_year  year  NULL,
    original_month  month  NULL,
    original_day  day  NULL
);

COMMENT ON TABLE release IS
    'The release date is the release of _this_ version of the release.  The'
    ' original release date is the first release of any version of the release.';

CREATE INDEX IF NOT EXISTS release_primary_artist_id ON release (primary_artist_id);
CREATE INDEX IF NOT EXISTS release_title ON release (title);

CREATE TABLE IF NOT EXISTS release_search_hint (
    release_id  UUID  NOT NULL
        REFERENCES release (release_id) ON UPDATE CASCADE ON DELETE CASCADE,
    hint  non_empty_citext  NOT NULL,
    PRIMARY KEY ( release_id, hint )
);

CREATE INDEX IF NOT EXISTS release_search_hint_hint ON release_search_hint (hint);

CREATE TABLE IF NOT EXISTS release_artist (
    release_id  UUID  NOT NULL
        REFERENCES release (release_id) ON UPDATE CASCADE ON DELETE CASCADE,
    artist_id  UUID  NOT NULL
        REFERENCES artist (artist_id) ON UPDATE CASCADE ON DELETE CASCADE,
    PRIMARY KEY ( release_id, artist_id )
);

CREATE TABLE IF NOT EXISTS track (
    track_id  UUID  PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    musicbrainz_track_id  INTEGER  UNIQUE NULL
        REFERENCES musicbrainz.track (id) ON UPDATE CASCADE ON DELETE CASCADE,
    primary_artist_id  UUID  NOT NULL
        REFERENCES artist (artist_id) ON UPDATE CASCADE ON DELETE CASCADE,
    title  non_empty_citext  NOT NULL,
    transcripted_title  non_empty_citext  NULL,
    translated_title  non_empty_citext  NULL,
    length  positive_int  NULL,
    content_hash  hash  NOT NULL
);

CREATE INDEX IF NOT EXISTS track_primary_artist_id ON track (primary_artist_id);
CREATE INDEX IF NOT EXISTS track_title ON track (title);

CREATE TABLE IF NOT EXISTS track_search_hint (
    track_id  UUID  NOT NULL
        REFERENCES track (track_id) ON UPDATE CASCADE ON DELETE CASCADE,
    hint  non_empty_citext  NOT NULL,
    PRIMARY KEY ( track_id, hint )
);

CREATE INDEX IF NOT EXISTS track_search_hint_hint ON track_search_hint (hint);

CREATE TABLE IF NOT EXISTS release_track (
    release_id  UUID  NOT NULL
         REFERENCES release (release_id) ON UPDATE CASCADE ON DELETE CASCADE,
    track_id  UUID  NOT NULL
         REFERENCES track (track_id) ON UPDATE CASCADE ON DELETE CASCADE,
    position  positive_int  NOT NULL,
    PRIMARY KEY ( release_id, track_id )
);

CREATE TABLE IF NOT EXISTS playlist (
    playlist_id  UUID  PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    title  non_empty_citext  NOT NULL
);

CREATE TABLE IF NOT EXISTS playlist_track (
    playlist_id  UUID  NOT NULL
         REFERENCES playlist (playlist_id) ON UPDATE CASCADE ON DELETE CASCADE,
    track_id  UUID  NOT NULL
         REFERENCES track (track_id) ON UPDATE CASCADE ON DELETE CASCADE,
    position  positive_int  NOT NULL,
    PRIMARY KEY ( playlist_id, track_id, position )
);

COMMENT ON TABLE playlist_track IS
    'The primary key of ( playlist_id, track_id, position ) allows a single'
    ' track to appear more than once on a playlist. Why not?';

CREATE TABLE IF NOT EXISTS user_track (
    user_id  UUID  NOT NULL
         REFERENCES "user" (user_id) ON UPDATE CASCADE ON DELETE CASCADE,
    track_id  UUID  NOT NULL
         REFERENCES track (track_id) ON UPDATE CASCADE ON DELETE CASCADE,
    upvotes  BIGINT  NOT NULL DEFAULT 0,
    downvotes  BIGINT  NOT NULL DEFAULT 0,
    PRIMARY KEY ( user_id, track_id )
);

-- XXX - in the final version we should make sure that for any given
-- user/client pair there are exactly 0 or 1 rows where is_current is true.
CREATE TABLE IF NOT EXISTS user_queue (
    user_id  UUID  NOT NULL
         REFERENCES "user" (user_id) ON UPDATE CASCADE ON DELETE CASCADE,
    client_id  UUID  NOT NULL,
    position  NUMERIC  NOT NULL,
    track_id  UUID  NOT NULL
         REFERENCES track (track_id) ON UPDATE CASCADE ON DELETE CASCADE,
    is_current  BOOL  DEFAULT false,
    PRIMARY KEY ( user_id, client_id, position )
);

COMMENT ON TABLE user_queue IS
    'The primary key of ( user_id, client_id, position ) allows a track to appear more'
    ' than once. Why not?';

COMMENT ON COLUMN user_queue.client_id IS
    'This is a totally arbitrary UUID that exists to allow the user to have'
    ' different queues on different devices. For mobile apps, we will want to'
    ' generate this value once at install time. For the web, I''m not sure what'
    ' the best approach is. We could simply hard code a single UUID for all web'
    ' clients, but that means that the queue will be shared by every device'
    ' running the web client. Is that okay?';

COMMENT ON COLUMN user_queue.position IS
    'We only need the queue.position column to be ordered. We do not require'
    ' that it increases in any specific amount between items. This lets us delete an item from the queue without'
    ' having to reorder all subsequent items. A "SELECT ... ORDER BY position"'
    ' will still give us the correct result. We can also insert items anywhere'
    ' in the queue by picking a position between the two surrounding items.';

COMMIT;

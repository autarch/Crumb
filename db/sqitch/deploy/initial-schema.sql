-- Deploy music-player:initial-schema to pg

BEGIN;

CREATE SCHEMA crumb;

SET search_path TO crumb;

CREATE EXTENSION IF NOT EXISTS citext;

CREATE DOMAIN non_empty_citext AS CITEXT
    CHECK ( VALUE != '' );

CREATE DOMAIN non_empty_text AS TEXT
    CHECK ( VALUE != '' );

-- Obviously this isn't a full email regex, but this is good enough for now.
CREATE DOMAIN email AS CITEXT
    CHECK ( VALUE ~ E'^[a-zA-Z0-9.!#$%&''*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$' );

CREATE DOMAIN hash AS TEXT
    CHECK ( VALUE ~ E'^\\$[a-z0-9_\-]+\\$[0-9a-f]+$' );

-- 1877 is the year of the first audio recording technology. Year + 1 for
-- something that someone imports on 12/31 that is a prerelease of something
-- from the next year.
CREATE DOMAIN year AS SMALLINT
    CHECK ( VALUE >= 1877 AND VALUE <= EXTRACT(YEAR FROM CURRENT_DATE) + 1 );

CREATE DOMAIN month AS SMALLINT
    CHECK ( VALUE >= 1 AND VALUE <= 12 );

CREATE DOMAIN day AS SMALLINT
    CHECK ( VALUE >= 1 AND VALUE <= 31 );

CREATE DOMAIN positive_smallint AS SMALLINT
    CHECK ( VALUE >= 1 );

CREATE DOMAIN positive_int AS INT
    CHECK ( VALUE >= 1 );

CREATE TYPE name_type AS ENUM ( 'original', 'transcripted', 'translated' );

CREATE TABLE "user" (
    user_id  UUID  PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    email  email  UNIQUE NOT NULL,
    date_format  non_empty_text  NOT NULL DEFAULT '%m-%d-%Y',
    preferred_name_order  name_type[]  NOT NULL  DEFAULT ARRAY['translated', 'transcripted', 'original']::name_type[],
    CONSTRAINT preferred_name_order_is_3 CHECK ( ARRAY_LENGTH(preferred_name_order, 1) == 3 )
);

CREATE TABLE artist (
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

CREATE INDEX artist_name ON artist (name);

CREATE TABLE artist_search_hint (
    artist_id  UUID  NOT NULL
        REFERENCES artist (artist_id) ON UPDATE CASCADE ON DELETE CASCADE,
    hint  non_empty_citext  NOT NULL,
    PRIMARY KEY ( artist_id, hint )
);

CREATE INDEX artist_search_hint_hint ON artist_search_hint (hint);

CREATE TABLE release (
    release_id  UUID  PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    musicbrainz_release_id  INTEGER  UNIQUE NULL
        REFERENCES musicbrainz.release (id) ON UPDATE CASCADE ON DELETE CASCADE,
    primary_artist_id  UUID  NOT NULL
        REFERENCES artist (artist_id) ON UPDATE CASCADE ON DELETE CASCADE,
    title  non_empty_citext  NOT NULL,
    transcripted_title  non_empty_citext  NULL,
    translated_title  non_empty_citext  NULL,
    comment non_empty_citext NULL,
    -- The release date is the release of _this_ version of the release.
    release_year  year  NULL,
    release_month  month  NULL,
    release_day  day  NULL,
    -- The original release date is the first release of any version of the
    -- release.
    original_year  year  NULL,
    original_month  month  NULL,
    original_day  day  NULL
);

CREATE INDEX release_primary_artist_id ON release (primary_artist_id);
CREATE INDEX release_title ON release (title);

CREATE TABLE release_search_hint (
    release_id  UUID  NOT NULL
        REFERENCES release (release_id) ON UPDATE CASCADE ON DELETE CASCADE,
    hint  non_empty_citext  NOT NULL,
    PRIMARY KEY ( release_id, hint )
);

CREATE INDEX release_search_hint_hint ON release_search_hint (hint);

CREATE TABLE release_artist (
    release_id  UUID  NOT NULL
        REFERENCES release (release_id) ON UPDATE CASCADE ON DELETE CASCADE,
    artist_id  UUID  NOT NULL
        REFERENCES artist (artist_id) ON UPDATE CASCADE ON DELETE CASCADE,
    PRIMARY KEY ( release_id, artist_id )
);

CREATE TABLE track (
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

CREATE INDEX track_primary_artist_id ON track (primary_artist_id);
CREATE INDEX track_title ON track (title);

CREATE TABLE track_search_hint (
    track_id  UUID  NOT NULL
        REFERENCES track (track_id) ON UPDATE CASCADE ON DELETE CASCADE,
    hint  non_empty_citext  NOT NULL,
    PRIMARY KEY ( track_id, hint )
);

CREATE INDEX track_search_hint_hint ON track_search_hint (hint);

CREATE TABLE release_track (
    release_id  UUID  NOT NULL
         REFERENCES release (release_id) ON UPDATE CASCADE ON DELETE CASCADE,
    track_id  UUID  NOT NULL
         REFERENCES track (track_id) ON UPDATE CASCADE ON DELETE CASCADE,
    position  positive_int  NOT NULL,
    PRIMARY KEY ( release_id, track_id )
);

CREATE TABLE playlist (
    playlist_id  UUID  PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    title  non_empty_citext  NOT NULL
);

CREATE TABLE playlist_track (
    playlist_id  UUID  NOT NULL
         REFERENCES playlist (playlist_id) ON UPDATE CASCADE ON DELETE CASCADE,
    track_id  UUID  NOT NULL
         REFERENCES track (track_id) ON UPDATE CASCADE ON DELETE CASCADE,
    position  positive_int  NOT NULL,
    -- This allows a single track to appear more than once on a playlist. Why
    -- not?
    PRIMARY KEY ( playlist_id, track_id, position )
);

CREATE TABLE user_track (
    user_id  UUID  NOT NULL
         REFERENCES "user" (user_id) ON UPDATE CASCADE ON DELETE CASCADE,
    track_id  UUID  NOT NULL
         REFERENCES track (track_id) ON UPDATE CASCADE ON DELETE CASCADE,
    upvotes  BIGINT  NOT NULL DEFAULT 0,
    downvotes  BIGINT  NOT NULL DEFAULT 0,
    PRIMARY KEY ( user_id, track_id )
);

CREATE DOMAIN sha_256 AS CITEXT
    CHECK ( VALUE ~ '^[0-9a-f]{64}$' );

CREATE TABLE track_file (
    track_file_id  UUID  PRIMARY KEY,
    track_id  UUID  NOT NULL
         REFERENCES track (track_id) ON UPDATE CASCADE ON DELETE CASCADE,
    checksum  sha_256  NOT NULL,
    UNIQUE ( track_id, checksum )
);

CREATE TABLE user_track_file (
    user_id  UUID  NOT NULL
         REFERENCES "user" (user_id) ON UPDATE CASCADE ON DELETE CASCADE,
    track_file_id  UUID  NOT NULL
         REFERENCES track_file (track_file_id) ON UPDATE CASCADE ON DELETE CASCADE,
    added  TIMESTAMP WITH TIME ZONE  NOT NULL,
    last_played  TIMESTAMP WITH TIME ZONE  NULL,
    PRIMARY KEY ( user_id, track_file_id )
);

COMMIT;

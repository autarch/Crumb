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
CREATE DOMAIN email AS TEXT
    CHECK ( VALUE ~ '\A[^@ \t\r\n]+@[^@ \t\r\n]+\.[^@ \t\r\n]+\z' );

CREATE DOMAIN storage_uri AS TEXT
    CHECK ( VALUE ~ '(?:file|https?)://.+' );

-- 1877 is the year of the first audio recording technology. Year + 1 for
-- something that someone imports on 12/31 that is a prerelease of something
-- from the next year.
CREATE DOMAIN year AS SMALLINT
    CHECK ( VALUE >= 1877 AND VALUE <= EXTRACT(YEAR FROM CURRENT_DATE) + 1 );

CREATE DOMAIN month AS SMALLINT
    CHECK ( VALUE >= 1 AND VALUE < 12 );

CREATE DOMAIN day AS SMALLINT
    CHECK ( VALUE >= 1 AND VALUE < 31 );

CREATE DOMAIN positive_smallint AS SMALLINT
    CHECK ( VALUE >= 1 );

CREATE DOMAIN positive_int AS INT
    CHECK ( VALUE >= 1 );

CREATE TYPE alias_type AS ENUM ( 'transcripted', 'translated', 'search_hint' );
CREATE TYPE name_type AS ENUM ( 'original', 'transcripted', 'translated' );

CREATE TABLE "user" (
    user_id  UUID  PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    email  email  UNIQUE NOT NULL,
    date_format  non_empty_text  NOT NULL DEFAULT '%m-%d-%Y',
    preferred_alias_order  name_type[]  NOT NULL  DEFAULT ARRAY['translated', 'transcripted', 'original']::name_type[]
);

CREATE TABLE artist (
    artist_id  UUID  PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    musicbrainz_id  INTEGER  UNIQUE NULL
        REFERENCES musicbrainz.artist (id) ON UPDATE CASCADE ON DELETE CASCADE,
    name  non_empty_citext  NOT NULL
);

CREATE INDEX artist_name ON artist (name);

CREATE TABLE artist_alias (
    artist_id  UUID  NOT NULL
        REFERENCES artist (artist_id) ON UPDATE CASCADE ON DELETE CASCADE,
    alias_type  alias_type  NOT NULL,
    alias  non_empty_citext  NOT NULL,
    sortable_alias  non_empty_citext  NOT NULL,
    PRIMARY KEY ( artist_id, alias_type, alias ),
    UNIQUE ( artist_id, alias_type, sortable_alias )
);

CREATE INDEX artist_alias_alias ON artist_alias (alias);

CREATE TABLE release (
    release_id  UUID  PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    musicbrainz_id  INTEGER  UNIQUE NULL
        REFERENCES musicbrainz.release (id) ON UPDATE CASCADE ON DELETE CASCADE,
    primary_artist_id  UUID  NOT NULL
        REFERENCES artist (artist_id) ON UPDATE CASCADE ON DELETE CASCADE,
    title  non_empty_citext  NOT NULL,
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

CREATE TABLE release_alias (
    release_id  UUID  NOT NULL
        REFERENCES release (release_id) ON UPDATE CASCADE ON DELETE CASCADE,
    alias_type  alias_type  NOT NULL,
    alias  non_empty_citext  NOT NULL,
    sortable_alias  non_empty_citext  NOT NULL,
    PRIMARY KEY ( release_id, alias_type, alias ),
    UNIQUE ( release_id, alias_type, sortable_alias )
);

CREATE INDEX release_alias_alias ON release_alias (alias);

CREATE TABLE release_artist (
    release_id  UUID  NOT NULL
        REFERENCES release (release_id) ON UPDATE CASCADE ON DELETE CASCADE,
    artist_id  UUID  NOT NULL
        REFERENCES artist (artist_id) ON UPDATE CASCADE ON DELETE CASCADE,
    PRIMARY KEY ( release_id, artist_id )
);

CREATE TABLE track (
    track_id  UUID  PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    musicbrainz_id  INTEGER  UNIQUE NULL
        REFERENCES musicbrainz.track (id) ON UPDATE CASCADE ON DELETE CASCADE,
    primary_artist_id  UUID  NOT NULL
        REFERENCES artist (artist_id) ON UPDATE CASCADE ON DELETE CASCADE,
    title  non_empty_citext  NOT NULL,
    length  positive_smallint  NOT NULL,
    storage_uri  storage_uri  NOT NULL
);

CREATE INDEX track_primary_artist_id ON track (primary_artist_id);
CREATE INDEX track_title ON track (title);

CREATE TABLE track_alias (
    track_id  UUID  NOT NULL
        REFERENCES track (track_id) ON UPDATE CASCADE ON DELETE CASCADE,
    alias_type  alias_type  NOT NULL,
    alias  non_empty_citext  NOT NULL,
    sortable_alias  non_empty_citext  NOT NULL,
    PRIMARY KEY ( track_id, alias_type, alias ),
    UNIQUE ( track_id, alias_type, sortable_alias )
);

CREATE INDEX track_alias_alias ON track_alias (alias);

CREATE TABLE release_track (
    release_id  UUID  NOT NULL
         REFERENCES release (release_id) ON UPDATE CASCADE ON DELETE CASCADE,
    track_id  UUID  NOT NULL
         REFERENCES track (track_id) ON UPDATE CASCADE ON DELETE CASCADE,
    track_number  positive_smallint  NOT NULL,
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
    track_order  positive_int  NOT NULL,
    -- This allows a single track to appear more than once on a playlist. Why
    -- not?
    PRIMARY KEY ( playlist_id, track_id, track_order )
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

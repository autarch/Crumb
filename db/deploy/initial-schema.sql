-- Deploy music-player:initial-schema to pg

BEGIN;

CREATE SCHEMA music_player;

SET search_path TO music_player;

CREATE EXTENSION IF NOT EXISTS citext;

-- Obviously this isn't a full email regex, but this is good enough for now.
CREATE DOMAIN email AS TEXT
    CHECK ( VALUE ~ '\A[^@ \t\r\n]+@[^@ \t\r\n]+\.[^@ \t\r\n]+\z' );

CREATE DOMAIN storage_uri AS TEXT
    CHECK ( VALUE ~ '(?:file|https?)://.+' );

-- 1877 is the year of the first audio recording technology.
CREATE DOMAIN year AS SMALLINT
    CHECK ( VALUE >= 1877 AND VALUE <= EXTRACT(YEAR FROM CURRENT_DATE) );

CREATE DOMAIN month AS SMALLINT
    CHECK ( VALUE >= 1 AND VALUE < 12 );

CREATE DOMAIN day AS SMALLINT
    CHECK ( VALUE >= 1 AND VALUE < 31 );

CREATE DOMAIN positive_smallint AS SMALLINT
    CHECK ( VALUE >= 1 );

CREATE DOMAIN positive_int AS INT
    CHECK ( VALUE >= 1 );

CREATE TABLE "user" (
    user_id  UUID   PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    email    email  UNIQUE NOT NULL
);

CREATE TABLE artist (
    artist_id                   UUID     PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    musicbrainz_id              INTEGER  UNIQUE NULL
        REFERENCES musicbrainz.artist (id) ON UPDATE CASCADE ON DELETE CASCADE,
    -- The artist's name in its original language.
    name                        citext   NOT NULL,
    -- The artist's sortable name in its original language.
    sortable_name               citext   NOT NULL,
    -- The artist's name transcripted to the Latin alphabet.
    transcripted_name           citext   NULL,
    -- The artist's sortable name transcripted to the Latin alphabet.
    transcripted_sortable_name  citext   NULL,
    -- The artist's name translated to English.
    translated_name             citext   NULL,
    -- The artist's sortable name translated to English.
    translated_sortable_name    citext   NULL
);

CREATE INDEX artist_name ON artist (name);
CREATE INDEX artist_sortable_name ON artist (sortable_name);
CREATE INDEX transcripted_artist_name ON artist (transcripted_name);
CREATE INDEX transcripted_artist_sortable_name ON artist (transcripted_sortable_name);
CREATE INDEX translated_artist_name ON artist (translated_name);
CREATE INDEX translated_artist_sortable_name ON artist (translated_sortable_name);

CREATE TABLE release (
    release_id                  UUID     PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    musicbrainz_id              INTEGER  UNIQUE NULL
        REFERENCES musicbrainz.release (id) ON UPDATE CASCADE ON DELETE CASCADE,
    primary_artist_id           UUID    NOT NULL
         REFERENCES artist (artist_id) ON DELETE CASCADE,
    -- The release's name in its original language.
    name                        citext  NOT NULL,
    -- The release's sortable name in its original language.
    sortable_name               citext  NOT NULL,
    -- The release's name transcripted to the Latin alphabet.
    transcripted_name           citext  NULL,
    -- The release's sortable name transcripted to the Latin alphabet.
    transcripted_sortable_name  citext  NULL,
    -- The release's name translated to English.
    translated_name             citext  NULL,
    -- The release's sortable name translated to English.
    translated_sortable_name    citext  NULL,
    -- The release date is the release of _this_ version of the release.
    year                        year    NULL,
    month                       month   NULL,
    day                         day     NULL,
    -- The original release date is the first release of any version of the
    -- release.
    original_year               year    NULL,
    original_month              month   NULL,
    original_day                day     NULL
);

CREATE INDEX release_name ON release (name);
CREATE INDEX release_sortable_name ON release (sortable_name);
CREATE INDEX transcripted_release_name ON release (transcripted_name);
CREATE INDEX transcripted_release_sortable_name ON release (transcripted_sortable_name);
CREATE INDEX translated_release_name ON release (translated_name);
CREATE INDEX translated_release_sortable_name ON release (translated_sortable_name);

CREATE TABLE "role" (
    role_id  UUID    PRIMARY KEY NOT NULL,
    name     citext  NOT NULL
);

CREATE TABLE release_artist (
    release_id  UUID  NOT NULL
        REFERENCES release (release_id) ON UPDATE CASCADE ON DELETE CASCADE,
    artist_id   UUID  NOT NULL
        REFERENCES artist (artist_id) ON UPDATE CASCADE ON DELETE CASCADE,
    role_id     UUID  NOT NULL
        REFERENCES "role" (role_id) ON UPDATE CASCADE ON DELETE CASCADE,
    PRIMARY KEY ( release_id, artist_id, role_id )
);

CREATE TABLE track (
    track_id                    UUID     PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    musicbrainz_id              INTEGER  UNIQUE NULL
        REFERENCES musicbrainz.track (id) ON UPDATE CASCADE ON DELETE CASCADE,
    primary_artist_id           UUID     NOT NULL
         REFERENCES artist (artist_id) ON DELETE CASCADE,
    -- The track's name in its original language.
    name                        citext   NOT NULL,
    -- The track's sortable name in its original language.
    sortable_name               citext   NOT NULL,
    -- The track's name transcripted to the Latin alphabet.
    transcripted_name           citext   NULL,
    -- The track's sortable name transcripted to the Latin alphabet.
    transcripted_sortable_name  citext   NULL,
    -- The track's name translated to English.
    translated_name             citext   NULL,
    -- The track's sortable name translated to English.
    translated_sortable_name    citext   NULL,
    -- The length of the track in seconds
    length                      positive_smallint  NOT NULL,
    storage_uri                 storage_uri        NOT NULL
);

CREATE TABLE user_track (
    user_id      UUID  NOT NULL
         REFERENCES "user" (user_id) ON DELETE CASCADE,
    track_id     UUID  NOT NULL
         REFERENCES track (track_id) ON DELETE CASCADE,
    added        TIMESTAMP WITH TIME ZONE  NOT NULL,
    last_played  TIMESTAMP WITH TIME ZONE  NULL
);

CREATE TABLE release_track (
    release_id    UUID  NOT NULL
         REFERENCES release (release_id) ON DELETE CASCADE,
    track_id      UUID  NOT NULL
         REFERENCES track (track_id) ON DELETE CASCADE,
    track_number  positive_smallint  NOT NULL,
    PRIMARY KEY ( release_id, track_id )
);

CREATE TABLE playlist (
    playlist_id  UUID    PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    title        citext  NOT NULL
);

CREATE TABLE playlist_track (
    playlist_id  UUID  NOT NULL
         REFERENCES playlist (playlist_id) ON DELETE CASCADE,
    track_id     UUID  NOT NULL
         REFERENCES track (track_id) ON DELETE CASCADE,
    track_order  positive_int  NOT NULL,
    -- This allows a single track to appear more than once on a playlist. Why
    -- not?
    PRIMARY KEY ( playlist_id, track_id, track_order )
);

COMMIT;

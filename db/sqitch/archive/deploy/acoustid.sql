-- Deploy crumb:acoustid to pg

-- Copied from the SQL generated by running:
--
--     PYTHONPATH=.:../mbdata/ alembic-- upgrade --sql head > sql
--
-- ... in the acoustid repo.

BEGIN;

CREATE SCHEMA acoustid;

SET search_path TO acoustid;

-- This removes a number of columns that aren't present in the data
-- dumps. AFAICT we only need the columns below to keep the data up to date
-- and to look up MB ids from a fingerprint.
CREATE TABLE fingerprint (
    id SERIAL NOT NULL, 
    fingerprint INTEGER[] NOT NULL, 
    length SMALLINT NOT NULL, 
    created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL, 
    track_id INTEGER NOT NULL, 
    CONSTRAINT fingerprint_pkey PRIMARY KEY (id)
);

CREATE INDEX fingerprint_idx_length ON fingerprint (length);

CREATE INDEX fingerprint_idx_track_id ON fingerprint (track_id);

CREATE TABLE track (
    id SERIAL NOT NULL, 
    created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL, 
    new_id INTEGER, 
    gid UUID NOT NULL, 
    CONSTRAINT track_pkey PRIMARY KEY (id)
);

CREATE UNIQUE INDEX track_idx_gid ON track (gid);

CREATE TABLE track_mbid (
    track_id INTEGER NOT NULL, 
    mbid UUID NOT NULL, 
    created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL, 
    id SERIAL NOT NULL, 
    submission_count INTEGER NOT NULL, 
    disabled BOOLEAN DEFAULT false NOT NULL, 
    CONSTRAINT track_mbid_pkey PRIMARY KEY (id)
);

CREATE INDEX track_mbid_idx_mbid ON track_mbid (mbid);

CREATE UNIQUE INDEX track_mbid_idx_uniq ON track_mbid (track_id, mbid);

ALTER TABLE fingerprint ADD CONSTRAINT fingerprint_fk_track_id FOREIGN KEY(track_id) REFERENCES track (id);

ALTER TABLE track ADD CONSTRAINT track_fk_new_id FOREIGN KEY(new_id) REFERENCES track (id);

ALTER TABLE track_mbid ADD CONSTRAINT track_mbid_fk_track_id FOREIGN KEY(track_id) REFERENCES track (id);

ALTER TABLE fingerprint ADD COLUMN updated TIMESTAMP WITH TIME ZONE;

ALTER TABLE track_mbid ADD COLUMN updated TIMESTAMP WITH TIME ZONE;

ALTER TABLE track ADD COLUMN updated TIMESTAMP WITH TIME ZONE;

COMMIT;

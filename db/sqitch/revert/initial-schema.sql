-- Revert music-player:initial-schema from pg

SET client_min_messages TO 'warning';

BEGIN;

DROP SCHEMA IF EXISTS crumb CASCADE;

COMMIT;

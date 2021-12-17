-- Revert crumb:acoustid from pg

SET client_min_messages TO 'warning';

BEGIN;

DROP SCHEMA IF EXISTS acoustid CASCADE;

COMMIT;

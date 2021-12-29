-- Revert crumb:functions-and-triggers from pg

BEGIN;

DROP FUNCTION IF EXISTS crumb.release_date_for_release(release crumb.release);
DROP FUNCTION IF EXISTS crumb.best_cover_image_for_artist( artist_id UUID, for_user_id UUID );
DROP FUNCTION IF EXISTS crumb.best_cover_image_for_release( mb_release_id INT, mb_release_group_id INT );
DROP FUNCTION IF EXISTS crumb.cover_image_url( mb_release_gid UUID, mb_cover_art_id BIGINT, mime_type TEXT );

COMMIT;

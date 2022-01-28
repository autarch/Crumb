CREATE OR REPLACE FUNCTION pg_temp.domain_exists(name TEXT)
    RETURNS BOOL
AS $$
DECLARE
    r BOOL = false;
BEGIN
    SELECT true INTO r
      FROM pg_type
     WHERE typname = name;
    RETURN r;
END;
$$ LANGUAGE plpgsql;

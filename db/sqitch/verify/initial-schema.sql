-- Verify music-player:initial-schema on pg

BEGIN;

CREATE TEMPORARY TABLE h (
    val  crumb.hash  NOT NULL
);

CREATE FUNCTION pg_temp.insert_h(val TEXT) RETURNS BOOL AS $$
BEGIN
    INSERT INTO h VALUES (val);
    RETURN true;
EXCEPTION
    WHEN OTHERS THEN
        RETURN false;
END;
$$ LANGUAGE plpgsql;

DO
$$
BEGIN
    ASSERT pg_temp.insert_h('$blake3$f5b5ee0a5f31f4bde6ae8d7ec2f9e7258ffad873eb5de2aeca826e98e12b9390');
    ASSERT pg_temp.insert_h('$md5$8b4a58380c7df4f477710327caee15a0');
    ASSERT NOT pg_temp.insert_h('');
    ASSERT NOT pg_temp.insert_h('hello!');
    ASSERT NOT pg_temp.insert_h('f5b5ee0a5f31f4bde6ae8d7ec2f9e7258ffad873eb5de2aeca826e98e12b9390');
END;
$$ LANGUAGE plpgsql;

ROLLBACK;

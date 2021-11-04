CREATE OR REPLACE FUNCTION trigger_set_timestamp ()
    RETURNS TRIGGER
    AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$
LANGUAGE plpgsql;

CREATE TABLE todos (
    id serial NOT NULL PRIMARY KEY,
    content text,
    created_at timestamptz NOT NULL DEFAULT NOW(),
    updated_at timestamptz NOT NULL DEFAULT NOW(),
    completed_at timestamptz
);

CREATE TRIGGER set_timestamp
    BEFORE UPDATE ON todos
    FOR EACH ROW
    EXECUTE PROCEDURE trigger_set_timestamp ();

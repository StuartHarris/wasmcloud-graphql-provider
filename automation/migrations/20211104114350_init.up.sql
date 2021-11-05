CREATE OR REPLACE FUNCTION trigger_set_timestamp ()
    RETURNS TRIGGER
    AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$
LANGUAGE plpgsql;

CREATE TABLE IF NOT EXISTS todos (
    id serial NOT NULL PRIMARY KEY,
    content text,
    created_at timestamptz NOT NULL DEFAULT NOW(),
    updated_at timestamptz NOT NULL DEFAULT NOW(),
    completed_at timestamptz
);

CREATE OR REPLACE TRIGGER set_timestamp BEFORE UPDATE ON todos FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp ();

COMMENT ON TABLE _sqlx_migrations IS E'@omit';

CREATE INDEX IF NOT EXISTS "todos_completed" ON "public"."todos" ("completed_at");

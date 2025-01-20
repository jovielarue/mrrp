-- This file was automatically created by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.




-- Sets up a trigger for the given table to automatically set a column called
-- `updated_at` whenever the row is modified (unless `updated_at` was included
-- in the modified columns)
--
-- # Example
--
-- ```sql
-- CREATE TABLE users (id SERIAL PRIMARY KEY, updated_at TIMESTAMP NOT NULL DEFAULT NOW());
--
-- SELECT diesel_manage_updated_at('users');
-- ```
CREATE OR REPLACE FUNCTION diesel_manage_updated_at(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION diesel_set_updated_at() RETURNS trigger AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;


-- This function works with our sequence to scramble its sequence value. Our sequences increment by 6, so we can store about 2,147,483,647/6 (357,913,941) different rows until we need to change this function to support BIGINTs. Currently, we cast the nextval which is a bigint (8 bytes) as an int (4 bytes).
-- Function from https://wiki.postgresql.org/wiki/Pseudo_encrypt
-- More info on numeric types: https://www.postgresql.org/docs/current/datatype-numeric.html
CREATE OR REPLACE FUNCTION pseudo_encrypt(value INTEGER) returns INTEGER AS $func$
DECLARE
l1 INTEGER;
l2 INTEGER;
r1 INTEGER;
r2 INTEGER;
i INTEGER:=0;
BEGIN
  l1 := (value >> 16) & 65535;
  r1 := value & 65535;
  WHILE i < 3 LOOP
    l2 := r1;
    r2 := l1 # ((((1366 * r1 + 150889) % 714025) / 714025.0) * 32767)::INTEGER;
    l1 := l2;
    r1 := r2;
    i := i + 1;
  END LOOP;
  return ((r1 << 16) + l1);
END;
$func$ LANGUAGE plpgsql strict immutable;


CREATE SEQUENCE IF NOT EXISTS seq_photo_id
AS INTEGER
INCREMENT BY 2
START WITH 1;

CREATE TABLE photos (
  photo_id INTEGER PRIMARY KEY UNIQUE NOT NULL DEFAULT pseudo_encrypt(CAST(nextval('seq_post_id') as INTEGER)),
  post_id INTEGER NOT NULL,
  description VARCHAR(128),
  photographer VARCHAR(128),
  photo_path VARCHAR(128) NOT NULL,
  time_taken timestamp with timezone NOT NULL,
  CONSTRAINT fk_post_id FOREIGN KEY (post_id) REFERENCES "posts" (post_id)
);

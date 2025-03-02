CREATE TABLE users (
  user_id INTEGER PRIMARY KEY,
  username VARCHAR(128) NOT NULL,
  password VARCHAR(128) NOT NULL
);

-- Trigger function to get rid of diesel's required post_id and insert DB's id
CREATE OR REPLACE FUNCTION fn_update_user_id() RETURNS trigger AS $update_user_id$
BEGIN

    NEW.user_id := nextval('seq_user_id');

  RETURN NEW;
END;
$update_user_id$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER t_update_user_id
BEFORE INSERT OR UPDATE ON users
FOR EACH ROW
EXECUTE FUNCTION fn_update_user_id();


CREATE SEQUENCE IF NOT EXISTS seq_post_id
AS INTEGER
INCREMENT BY 1
START WITH 1;

CREATE TABLE posts (
  post_id INTEGER PRIMARY KEY,
  user_id INTEGER NOT NULL,
  text VARCHAR(2048) NOT NULL,
  like_count INTEGER,
  time timestamptz NOT NULL DEFAULT (now() at time zone 'utc'),
  CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES "users" (user_id)
);

-- Trigger function to get rid of diesel's required post_id and insert DB's id
CREATE OR REPLACE FUNCTION fn_update_post_id() RETURNS trigger AS $update_post_id$
BEGIN

    NEW.post_id := nextval('seq_post_id');

  RETURN NEW;
END;
$update_post_id$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER t_update_post_id
BEFORE INSERT ON posts
FOR EACH ROW
EXECUTE FUNCTION fn_update_post_id();

CREATE SEQUENCE IF NOT EXISTS seq_user_id
AS INTEGER
INCREMENT BY 1
START WITH 1;


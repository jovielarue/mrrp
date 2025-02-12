CREATE SEQUENCE IF NOT EXISTS seq_post_id
AS INTEGER
INCREMENT BY 1
START WITH 1;

CREATE TABLE posts (
  post_id INTEGER PRIMARY KEY,
  username VARCHAR(128) NOT NULL,
  text VARCHAR(2048) NOT NULL,
  like_count INTEGER,
  time timestamptz NOT NULL DEFAULT (now() at time zone 'utc')
);

CREATE SEQUENCE IF NOT EXISTS seq_photo_id
AS INTEGER
INCREMENT BY 1
START WITH 1;

CREATE TABLE photos (
  photo_id INTEGER PRIMARY KEY UNIQUE DEFAULT nextval('seq_post_id'),
  post_id INTEGER NOT NULL,
  description VARCHAR(128),
  photographer VARCHAR(128),
  photo_path VARCHAR(128) NOT NULL,
  CONSTRAINT fk_post_id FOREIGN KEY (post_id) REFERENCES "posts" (post_id)
);

-- Trigger function to get rid of diesel's required post_id and insert DB's id
CREATE OR REPLACE FUNCTION fn_update_post_id() RETURNS trigger AS $update_post_id$
BEGIN

    NEW.post_id := nextval('seq_post_id');

  RETURN NEW;
END;
$update_post_id$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER t_update_post_id
BEFORE INSERT OR UPDATE ON posts
FOR EACH ROW
EXECUTE FUNCTION fn_update_post_id();

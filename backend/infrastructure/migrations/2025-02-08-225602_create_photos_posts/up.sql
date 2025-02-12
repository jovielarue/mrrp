CREATE SEQUENCE IF NOT EXISTS seq_post_id
AS INTEGER
INCREMENT BY 2
START WITH 2;

CREATE TABLE posts (
  post_id INTEGER PRIMARY KEY UNIQUE NOT NULL DEFAULT nextval('seq_post_id'),
  text VARCHAR(2048),
  like_count INTEGER,
  time timestamptz NOT NULL DEFAULT (now() at time zone 'utc')
);

CREATE SEQUENCE IF NOT EXISTS seq_photo_id
AS INTEGER
INCREMENT BY 2
START WITH 1;

CREATE TABLE photos (
  photo_id INTEGER PRIMARY KEY UNIQUE DEFAULT nextval('seq_post_id'),
  post_id INTEGER NOT NULL,
  description VARCHAR(128),
  photographer VARCHAR(128),
  photo_path VARCHAR(128) NOT NULL,
  CONSTRAINT fk_post_id FOREIGN KEY (post_id) REFERENCES "posts" (post_id)
);

-- Your SQL goes here
ALTER TABLE users ADD CONSTRAINT unique_user_alias UNIQUE (alias);

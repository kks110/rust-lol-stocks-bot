-- Your SQL goes here
ALTER TABLE users ADD CONSTRAINT unique_user_name UNIQUE (name);
ALTER TABLE teams ADD CONSTRAINT unique_team_name UNIQUE (name);
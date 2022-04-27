-- Your SQL goes here
ALTER TABLE users ADD CONSTRAINT unique_user_discord_id UNIQUE (discord_id);

-- Your SQL goes here
CREATE TABLE leagues (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE
);

ALTER TABLE teams ADD COLUMN league_id INTEGER references leagues(id);
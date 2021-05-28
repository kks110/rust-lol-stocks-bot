-- Your SQL goes here
CREATE TABLE teams (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    elo INTEGER NOT NULL
)
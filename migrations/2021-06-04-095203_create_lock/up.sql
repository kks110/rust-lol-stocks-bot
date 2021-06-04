-- Your SQL goes here
CREATE TABLE locks (
    id SERIAL PRIMARY KEY,
    locked BOOLEAN NOT NULL
);

INSERT INTO locks (locked) VALUES (false)
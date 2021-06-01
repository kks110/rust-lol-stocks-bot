-- Your SQL goes here
CREATE TABLE portfolios (
    id SERIAL PRIMARY KEY,
    team_id INTEGER NOT NULL references teams(id),
    user_id INTEGER NOT NULL references users(id),
    amount INTEGER NOT NULL
)

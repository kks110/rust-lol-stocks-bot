-- Your SQL goes here
CREATE TABLE team_elo_histories (
    id SERIAL PRIMARY KEY,
    date DATE NOT NULL DEFAULT CURRENT_DATE - INTEGER '1',
    elo INTEGER NOT NULL,
    team_id INTEGER NOT NULL references teams(id)
);

CREATE TABLE user_portfolio_histories (
    id SERIAL PRIMARY KEY,
    date DATE NOT NULL DEFAULT CURRENT_DATE - INTEGER '1',
    value INTEGER NOT NULL,
    user_id INTEGER NOT NULL references users(id)
);

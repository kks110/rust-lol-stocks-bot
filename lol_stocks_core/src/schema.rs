table! {
    locks (id) {
        id -> Int4,
        locked -> Bool,
    }
}

table! {
    portfolios (id) {
        id -> Int4,
        team_id -> Int4,
        user_id -> Int4,
        amount -> Int4,
    }
}

table! {
    team_elo_histories (id) {
        id -> Int4,
        date -> Date,
        elo -> Int4,
        team_id -> Int4,
    }
}

table! {
    teams (id) {
        id -> Int4,
        name -> Varchar,
        elo -> Int4,
    }
}

table! {
    user_portfolio_histories (id) {
        id -> Int4,
        date -> Date,
        value -> Int4,
        user_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        balance -> Int4,
        admin -> Bool,
    }
}

joinable!(portfolios -> teams (team_id));
joinable!(portfolios -> users (user_id));
joinable!(team_elo_histories -> teams (team_id));
joinable!(user_portfolio_histories -> users (user_id));

allow_tables_to_appear_in_same_query!(
    locks,
    portfolios,
    team_elo_histories,
    teams,
    user_portfolio_histories,
    users,
);

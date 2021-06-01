table! {
    portfolios (id) {
        id -> Int4,
        team_id -> Int4,
        user_id -> Int4,
        amount -> Int4,
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
    users (id) {
        id -> Int4,
        name -> Varchar,
        balance -> Int4,
    }
}

joinable!(portfolios -> teams (team_id));
joinable!(portfolios -> users (user_id));

allow_tables_to_appear_in_same_query!(
    portfolios,
    teams,
    users,
);

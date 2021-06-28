use diesel::prelude::*;
use crate::models::league::{League, NewLeague};

pub fn load_leagues(conn: &PgConnection) -> Vec<League>  {
    use crate::schema::leagues::dsl::*;

    leagues.load::<League>(conn).expect("Error loading teams")
}

pub fn load_league(conn: &PgConnection, league_name: &str) -> League {
    use crate::schema::leagues::dsl::*;
    let uppercase_league_name = league_name.to_uppercase();

    leagues.filter(name.eq(&uppercase_league_name))
        .first(conn)
        .expect("Error loading league")
}

pub fn create_league<'a>(conn: &PgConnection, name: &'a str) -> League {
    use crate::schema::leagues;

    let uppercase_league_name = name.to_uppercase();

    let new_league = NewLeague {
        name: &uppercase_league_name,
    };

    diesel::insert_into(leagues::table)
        .values(&new_league)
        .get_result(conn)
        .expect("Error saving league")
}

pub fn find_or_create_league(conn: &PgConnection, name: &str) -> League {
    let league_list = load_leagues(conn);
    for league in league_list {
        if league.name == name.to_uppercase() {
            return league
        }
    }
    create_league(conn, name)
}

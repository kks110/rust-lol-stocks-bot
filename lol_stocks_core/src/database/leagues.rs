use diesel::prelude::*;
use crate::models::league::{League, NewLeague};

pub fn load_leagues(conn: &PgConnection) -> Result<Vec<League>, String>  {
    use crate::schema::leagues::dsl::*;

    match leagues.load::<League>(conn) {
        Ok(returned_leagues) => Ok(returned_leagues),
        Err(_) => Err("Could not load leagues".to_string())
    }
}

pub fn load_league(conn: &PgConnection, league_name: &str) -> Result<League, String> {
    use crate::schema::leagues::dsl::*;
    let uppercase_league_name = league_name.to_uppercase();

    match leagues.filter(name.eq(&uppercase_league_name)).first(conn) {
        Ok(league) => Ok(league),
        Err(error) => Err(error.to_string())
    }

}

pub fn create_league<'a>(conn: &PgConnection, name: &'a str) -> Result<League, String> {
    use crate::schema::leagues;

    let uppercase_league_name = name.to_uppercase();

    let new_league = NewLeague {
        name: &uppercase_league_name,
    };

    match diesel::insert_into(leagues::table)
        .values(&new_league)
        .get_result(conn) {
        Ok(league) => Ok(league),
        Err(error) => Err(error.to_string())
    }

}

pub fn find_or_create_league(conn: &PgConnection, name: &str) -> Result<League, String> {
    let league_list = match load_leagues(conn) {
        Ok(l) => l,
        Err(e) => return Err(e)
    };
    for league in league_list {
        if league.name == name.to_uppercase() {
            return Ok(league)
        }
    }
    return match create_league(conn, name) {
        Ok(l) => Ok(l),
        Err(e) => Err(e)
    }
}

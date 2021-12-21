use diesel::prelude::*;
use crate::models::league::{League, NewLeague};
use std::error::Error;

pub fn load_leagues(conn: &PgConnection) -> Result<Vec<League>, Box<dyn Error>>  {
    use crate::schema::leagues::dsl::*;

    Ok(leagues.load::<League>(conn)?)
}

pub fn load_league(conn: &PgConnection, league_name: &str) -> Result<League, Box<dyn Error>> {
    use crate::schema::leagues::dsl::*;
    let uppercase_league_name = league_name.to_uppercase();

    Ok(leagues.filter(name.eq(&uppercase_league_name)).first(conn)?)
}

pub fn create_league(conn: &PgConnection, name: &str) -> Result<League, Box<dyn Error>> {
    use crate::schema::leagues;

    let uppercase_league_name = name.to_uppercase();

    let new_league = NewLeague {
        name: &uppercase_league_name,
    };

    Ok(diesel::insert_into(leagues::table)
        .values(&new_league)
        .get_result(conn)?
    )

}

pub fn find_or_create_league(conn: &PgConnection, name: &str) -> Result<League, Box<dyn Error>> {
    let league_list = load_leagues(conn)?;

    for league in league_list {
        if league.name == name.to_uppercase() {
            return Ok(league)
        }
    }
    create_league(conn, name)
}

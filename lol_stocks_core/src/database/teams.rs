use diesel::prelude::*;
use crate::models::team::{Team, NewTeam};
use crate::database::leagues::load_league;
use crate::database::leagues::find_or_create_league;

pub fn load_teams(conn: &PgConnection) -> Result<Vec<Team>, String>  {
    use crate::schema::teams::dsl::*;

    match teams.order(elo.desc()).load::<Team>(conn) {
        Ok(ts) => Ok(ts),
        Err(e) => Err(e.to_string())
    }
}

pub fn load_teams_by_league(conn: &PgConnection, league_name: &str) -> Result<Vec<Team>, String>  {
    use crate::schema::teams::dsl::*;

    let league = match load_league(&conn, league_name) {
        Ok(l) => l,
        Err(e) => return Err(e)
    };

    match teams.filter(league_id.eq(&league.id))
        .order(elo.desc())
        .load::<Team>(conn) {
        Ok(team) => Ok(team),
        Err(e) => Err(e.to_string())
    }
}

pub fn load_team(conn: &PgConnection, team_name: &str) -> Result<Team, String> {
    use crate::schema::teams::dsl::*;
    let uppercase_team_name = team_name.to_uppercase();

    match teams.filter(name.eq(&uppercase_team_name))
        .first(conn) {
        Ok(team) => Ok(team),
        Err(e) => Err(e.to_string())
    }
}

pub fn load_team_by_id(conn: &PgConnection, team_id: &i32) -> Result<Team, String> {
    use crate::schema::teams::dsl::*;

    match teams.filter(id.eq(team_id))
        .first(conn) {
        Ok(team) => Ok(team),
        Err(e) => Err(e.to_string())
    }
}

pub fn update_team<'a>(conn: &PgConnection, team_name: &str, new_elo: i32) -> Result<Team, String> {
    use crate::schema::teams::dsl::*;
    let uppercase_team_name = team_name.to_uppercase();

    match diesel::update(teams.filter(name.eq(&uppercase_team_name)))
        .set(elo.eq(new_elo))
        .get_result::<Team>(conn) {
        Ok(team) => Ok(team),
        Err(e) => Err(e.to_string())
    }
}

pub fn create_team<'a>(conn: &PgConnection, name: &'a str, league_name: &'a str) -> Result<Team, String> {
    use crate::schema::teams;

    let league = match find_or_create_league(&conn, league_name) {
        Ok(l) => l,
        Err(e) => return Err(e)
    };

    let new_team = NewTeam {
        name,
        elo: &500,
        league_id: &league.id
    };

    return match diesel::insert_into(teams::table)
        .values(&new_team)
        .get_result(conn) {
        Ok(team) => Ok(team),
        Err(e) => Err(e.to_string())
    }
}

use diesel::prelude::*;
use crate::models::team::{Team, NewTeam};
use crate::database::leagues::load_league;
use crate::database::leagues::find_or_create_league;
use std::error::Error;

pub fn load_teams(conn: &PgConnection) -> Result<Vec<Team>, Box<dyn Error>>  {
    use crate::schema::teams::dsl::*;

    Ok(teams.order(elo.desc()).load::<Team>(conn)?)
}

pub fn load_teams_by_league(conn: &PgConnection, league_name: &str) -> Result<Vec<Team>, Box<dyn Error>>  {
    use crate::schema::teams::dsl::*;

    let league = load_league(conn, league_name)?;

    Ok(teams.filter(league_id.eq(&league.id))
        .order(elo.desc())
        .load::<Team>(conn)?
    )
}

pub fn load_team(conn: &PgConnection, team_name: &str) -> Result<Team, Box<dyn Error>> {
    use crate::schema::teams::dsl::*;
    let uppercase_team_name = team_name.to_uppercase();

    Ok(teams.filter(name.eq(&uppercase_team_name))
        .first(conn)?
    )
}

pub fn load_team_by_id(conn: &PgConnection, team_id: &i32) -> Result<Team, Box<dyn Error>> {
    use crate::schema::teams::dsl::*;

    Ok(teams.filter(id.eq(team_id))
        .first(conn)?
    )
}

pub fn update_team(conn: &PgConnection, team_name: &str, new_elo: i32) -> Result<Team, Box<dyn Error>> {
    use crate::schema::teams::dsl::*;
    let uppercase_team_name = team_name.to_uppercase();

    Ok(diesel::update(teams.filter(name.eq(&uppercase_team_name)))
        .set(elo.eq(new_elo))
        .get_result::<Team>(conn)?
    )
}

pub fn create_team(conn: &PgConnection, name: &str, league_name: &str) -> Result<Team, Box<dyn Error>> {
    use crate::schema::teams;

    let league = find_or_create_league(conn, league_name)?;

    let new_team = NewTeam::new(name,league.id);

    Ok(diesel::insert_into(teams::table)
        .values(&new_team)
        .get_result(conn)?
    )
}

use diesel::prelude::*;
use crate::models::team::{Team, NewTeam};
use crate::database::leagues::load_league;
use crate::database::leagues::find_or_create_league;

pub fn load_teams(conn: &PgConnection) -> Vec<Team>  {
    use crate::schema::teams::dsl::*;

    teams.order(elo.desc()).load::<Team>(conn).expect("Error loading teams")
}

pub fn load_teams_be_league(conn: &PgConnection, league_name: &str) -> Vec<Team>  {
    use crate::schema::teams::dsl::*;

    let league = load_league(&conn, league_name);

    teams.filter(league_id.eq(&league.id))
        .load::<Team>(conn)
        .expect("Error loading teams")
}

pub fn load_team(conn: &PgConnection, team_name: &str) -> Team {
    use crate::schema::teams::dsl::*;
    let uppercase_team_name = team_name.to_uppercase();

    teams.filter(name.eq(&uppercase_team_name))
        .first(conn)
        .expect("Error loading team")
}

pub fn load_team_by_id(conn: &PgConnection, team_id: &i32) -> Team {
    use crate::schema::teams::dsl::*;

    teams.filter(id.eq(team_id))
        .first(conn)
        .expect("Error loading team")
}

pub fn update_team<'a>(conn: &PgConnection, team_name: &str, new_elo: i32) -> Team {
    use crate::schema::teams::dsl::*;
    let uppercase_team_name = team_name.to_uppercase();

    let team = diesel::update(teams.filter(name.eq(&uppercase_team_name)))
        .set(elo.eq(new_elo))
        .get_result::<Team>(conn)
        .expect(&format!("Unable to find team {}", team_name));
    return team;
}

pub fn create_team<'a>(conn: &PgConnection, name: &'a str, league_name: &'a str) -> Team {
    use crate::schema::teams;

    let league = find_or_create_league(&conn, league_name);

    let new_team = NewTeam {
        name,
        elo: &500,
        league_id: &league.id
    };

    diesel::insert_into(teams::table)
        .values(&new_team)
        .get_result(conn)
        .expect("Error saving team")
}

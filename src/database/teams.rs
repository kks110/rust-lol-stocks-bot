use crate::diesel::prelude::*;
use crate::models::team::{Team};

pub fn load_teams(conn: &PgConnection) -> Vec<Team>  {
    use crate::schema::teams::dsl::*;

    teams.load::<Team>(conn).expect("Error loading teams")
}

pub fn load_team(conn: &PgConnection, team_name: &str) -> Team {
    use crate::schema::teams::dsl::*;

    teams.filter(name.eq(team_name))
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

    let team = diesel::update(teams.filter(name.eq(team_name)))
        .set(elo.eq(new_elo))
        .get_result::<Team>(conn)
        .expect(&format!("Unable to find team {}", team_name));
    return team;
}

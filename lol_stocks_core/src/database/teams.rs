use diesel::prelude::*;
use crate::models::team::{Team, NewTeam};

pub fn load_teams(conn: &PgConnection) -> Vec<Team>  {
    use crate::schema::teams::dsl::*;

    teams.order(elo.desc()).load::<Team>(conn).expect("Error loading teams")
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

pub fn create_team<'a>(conn: &PgConnection, name: &'a str) -> Team {
    use crate::schema::teams;

    let new_team = NewTeam {
        name,
        elo: &500
    };

    diesel::insert_into(teams::table)
        .values(&new_team)
        .get_result(conn)
        .expect("Error saving team")
}

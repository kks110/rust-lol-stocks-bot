use diesel::prelude::*;
use crate::models::team_elo_history::{TeamEloHistory, NewTeamEloHistory};
use crate::models::team::Team;
use std::error::Error;

pub fn load_team_elo_history(conn: &PgConnection, team: &Team) -> Result<Vec<TeamEloHistory>, Box<dyn Error>> {
    use crate::schema::team_elo_histories::dsl::*;

    Ok(TeamEloHistory::belonging_to(team)
        .order(date.desc())
        .load::<TeamEloHistory>(conn)?
    )
}

pub fn create_team_elo_history(conn: &PgConnection, elo: i32, team_id: i32) -> Result<TeamEloHistory, Box<dyn Error>> {
    use crate::schema::team_elo_histories;

    let new_team_elo_history = NewTeamEloHistory::new(elo, team_id);

    Ok(diesel::insert_into(team_elo_histories::table)
        .values(&new_team_elo_history)
        .get_result(conn)?
    )
}

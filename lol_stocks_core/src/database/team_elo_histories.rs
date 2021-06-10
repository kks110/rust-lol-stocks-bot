use diesel::prelude::*;
use crate::models::team_elo_history::{TeamEloHistory, NewTeamEloHistory};
use crate::models::team::Team;

pub fn load_team_elo_history(conn: &PgConnection, team: &Team) -> Vec<TeamEloHistory> {
    use crate::schema::team_elo_histories::dsl::*;

    TeamEloHistory::belonging_to(team)
        .order(date.desc())
        .limit(5)
        .load::<TeamEloHistory>(conn)
        .expect("Error loading team elo history")
}

pub fn create_team_elo_history<'a>(conn: &PgConnection, elo: &'a i32, team_id: &'a i32) -> TeamEloHistory {
    use crate::schema::team_elo_histories;

    let new_team_elo_history = NewTeamEloHistory {
        elo,
        team_id
    };

    diesel::insert_into(team_elo_histories::table)
        .values(&new_team_elo_history)
        .get_result(conn)
        .expect("Error saving elo history")
}

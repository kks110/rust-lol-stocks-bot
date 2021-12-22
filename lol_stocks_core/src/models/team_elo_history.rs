use crate::schema::team_elo_histories;
use crate::models::team::Team;
use chrono::NaiveDate;

#[derive(Identifiable, Queryable, Associations)]
#[table_name="team_elo_histories"]
#[belongs_to(Team)]
pub struct TeamEloHistory {
    pub id: i32,
    pub date: NaiveDate,
    pub elo: i32,
    pub team_id: i32,
}

#[derive(Insertable)]
#[table_name="team_elo_histories"]
pub struct NewTeamEloHistory {
    pub elo: i32,
    pub team_id: i32,
}

impl NewTeamEloHistory {
    pub fn new(elo: i32, team_id: i32) -> NewTeamEloHistory {
        NewTeamEloHistory {
            elo,
            team_id
        }
    }
}
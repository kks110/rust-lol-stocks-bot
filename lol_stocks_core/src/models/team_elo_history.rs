use crate::schema::team_elo_histories;
use crate::models::team::Team;

#[derive(Identifiable, Queryable, Associations)]
#[table_name="team_elo_histories"]
#[belongs_to(Team)]
pub struct TeamEloHistory {
    pub id: i32,
    pub week: i32,
    pub elo: i32,
    pub team_id: i32,
}

#[derive(Insertable)]
#[table_name="team_elo_histories"]
pub struct NewTeamEloHistory<'a> {
    pub week: &'a i32,
    pub elo: &'a i32,
    pub team_id: &'a i32,
}
use crate::schema::teams;
use serde::{Serialize, Deserialize};

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub elo: i32,
    pub league_id: i32
}

#[derive(Insertable)]
#[table_name="teams"]
pub struct NewTeam<'a> {
    pub name: &'a str,
    pub elo: &'a i32,
    pub league_id: &'a i32,
}

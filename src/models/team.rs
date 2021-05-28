// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct TeamsList {
//     pub teams: Vec<Team>,
// }

use crate::schema::teams;

// #[derive(Serialize, Deserialize, PartialEq, Clone, Queryable)]
#[derive(Queryable)]
// #[serde(rename_all = "camelCase")]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub elo: i32
}

#[derive(Insertable)]
#[table_name="teams"]
pub struct NewTeam<'a> {
    pub name: &'a str,
    pub elo: &'a i32,
}
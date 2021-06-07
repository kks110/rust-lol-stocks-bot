use crate::schema::teams;

#[derive(Identifiable, Queryable)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub elo: i32
}

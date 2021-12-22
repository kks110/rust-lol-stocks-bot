use crate::schema::teams;

#[derive(Identifiable, Queryable, Associations)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub elo: i32,
    pub league_id: i32
}

#[derive(Insertable)]
#[table_name="teams"]
pub struct NewTeam {
    pub name: String,
    pub elo: i32,
    pub league_id: i32,
}

impl NewTeam {
    pub fn new(name: &str, league_id: i32) -> NewTeam {
        NewTeam {
            name: String::from(name),
            elo: 500,
            league_id
        }
    }
}
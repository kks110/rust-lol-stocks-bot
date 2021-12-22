use crate::schema::leagues;

#[derive(Identifiable, Queryable)]
pub struct League {
    pub id: i32,
    pub name: String
}

#[derive(Insertable)]
#[table_name="leagues"]
pub struct NewLeague {
    pub name: String
}

impl NewLeague {
    pub fn new(name: &str) -> NewLeague {
        NewLeague {
            name: String::from(name),
        }
    }
}
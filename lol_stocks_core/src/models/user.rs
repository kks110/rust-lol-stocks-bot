use crate::schema::users;

#[derive(Identifiable, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub balance: i32,
    pub admin: bool,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub name: String,
    pub balance: i32,
}

impl NewUser {
    pub fn new(name: &str) -> NewUser {
        NewUser {
            name: String::from(name),
            balance: 5000
        }
    }
}
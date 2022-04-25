use crate::schema::users;
use bigdecimal::BigDecimal;

#[derive(Identifiable, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub balance: i32,
    pub admin: bool,
    pub discord_id: BigDecimal,
    pub alias: Option<String>,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub name: String,
    pub balance: i32,
    pub discord_id: BigDecimal,
    pub alias: Option<String>
}

impl NewUser {
    pub fn new(name: &str, discord_id: BigDecimal, alias: Option<String>) -> NewUser {
        NewUser {
            name: String::from(name),
            balance: 5000,
            discord_id,
            alias
        }
    }
}

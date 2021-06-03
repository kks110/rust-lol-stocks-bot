use crate::schema::users;

#[derive(Identifiable, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub balance: i32
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub balance: &'a i32,
}
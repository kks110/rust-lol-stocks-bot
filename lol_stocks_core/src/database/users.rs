use diesel::prelude::*;
use crate::models::user::{User, NewUser};
use std::error::Error;

pub fn load_users(conn: &PgConnection) -> Result<Vec<User>, Box<dyn Error>>  {
    use crate::schema::users::dsl::*;

    Ok(users.load::<User>(conn)?)
}

pub fn load_user(conn: &PgConnection, user_name: &str) -> Result<User, Box<dyn Error>> {
    use crate::schema::users::dsl::*;

    Ok(users.filter(name.eq(user_name))
        .first(conn)?)
}

pub fn create_user(conn: &PgConnection, name: &str) -> Result<User, Box<dyn Error>> {
    use crate::schema::users;

    let new_user = NewUser {
        name,
        balance: &5000,
    };

    Ok(diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)?
    )
}

pub fn update_user(conn: &PgConnection, user_name: &str, new_balance: i32) -> Result<User, Box<dyn Error>> {
    use crate::schema::users::dsl::*;

    Ok(diesel::update(users.filter(name.eq(user_name)))
        .set(balance.eq(new_balance))
        .get_result::<User>(conn)?
    )
}

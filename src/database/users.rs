use crate::diesel::prelude::*;
use crate::models::user::{User, NewUser};

pub fn load_user(conn: &PgConnection, user_name: &str) -> User {
    use crate::schema::users::dsl::*;

    users.filter(name.eq(user_name))
        .first(conn)
        .expect("Error loading team")
}

pub fn create_user<'a>(conn: &PgConnection, name: &'a str) -> User {
    use crate::schema::users;

    let new_user = NewUser {
        name,
        balance: &5000,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn update_user<'a>(conn: &PgConnection, user_name: &str, new_balance: i32) -> User {
    use crate::schema::users::dsl::*;

    let user = diesel::update(users.filter(name.eq(user_name)))
        .set(balance.eq(new_balance))
        .get_result::<User>(conn)
        .expect(&format!("Unable to find user {}", user_name));
    return user;
}

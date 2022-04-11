use diesel::prelude::*;
use crate::models::user::{User, NewUser};
use std::error::Error;
use bigdecimal::{BigDecimal, FromPrimitive};
use crate::errors::{AliasSameAsAUserName, DiscordIdConversionError};

pub fn load_users(conn: &PgConnection) -> Result<Vec<User>, Box<dyn Error>>  {
    use crate::schema::users::dsl::*;

    Ok(users.load::<User>(conn)?)
}

pub fn load_user(conn: &PgConnection, user_name: &str) -> Result<User, Box<dyn Error>> {
    use crate::schema::users::dsl::*;

    Ok(users.filter(name.eq(user_name))
        .first(conn)?)
}

pub fn load_user_by_id(conn: &PgConnection, user_id: &i32) -> Result<User, Box<dyn Error>> {
    use crate::schema::users::dsl::*;

    Ok(users.filter(id.eq(user_id))
        .first(conn)?)
}

pub fn load_user_by_alias(conn: &PgConnection, user_alias: &str) -> Result<User, Box<dyn Error>> {
    use crate::schema::users::dsl::*;

    Ok(users.filter(alias.eq(user_alias))
        .first(conn)?)
}

pub fn load_user_by_discord_id(conn: &PgConnection, discord_id_number: &u64) -> Result<User, Box<dyn Error>> {
    use crate::schema::users::dsl::*;

    let numeric_discord_id = BigDecimal::from_u64(*discord_id_number)
        .ok_or_else(|| { DiscordIdConversionError::new() })?;

    Ok(users.filter(discord_id.eq(numeric_discord_id))
        .first(conn)?)
}

pub fn create_user(conn: &PgConnection, name: &str, discord_id: &u64, alias: Option<String>) -> Result<User, Box<dyn Error>> {
    use crate::schema::users;

    let numeric_discord_id = BigDecimal::from_u64(*discord_id)
        .ok_or_else(|| { DiscordIdConversionError::new() })?;

    if alias.is_some() {
        let a = alias.as_ref().unwrap();
        let users_list = load_users(conn)?;
        for user in users_list {
            if &user.name == a {
                return Err(Box::new(AliasSameAsAUserName::new()))
            }
        }
    }

    let new_user = NewUser::new(name, numeric_discord_id, alias);

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

pub fn make_user_admin(conn: &PgConnection, user_name: &str) -> Result<User, Box<dyn Error>> {
    use crate::schema::users::dsl::*;

    Ok(diesel::update(users.filter(name.eq(user_name)))
        .set(admin.eq(true))
        .get_result::<User>(conn)?
    )
}

pub fn add_user_alias(conn: &PgConnection, user_name: &str, user_alias: &str) -> Result<User, Box<dyn Error>> {
    use crate::schema::users::dsl::*;

    let users_list = load_users(conn)?;

    for user in users_list {
        if user.name == user_alias {
            return Err(Box::new(AliasSameAsAUserName::new()))
        }
    }

    Ok(diesel::update(users.filter(name.eq(user_name)))
        .set(alias.eq(user_alias))
        .get_result::<User>(conn)?
    )
}

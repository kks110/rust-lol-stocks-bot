use diesel::prelude::*;
use crate::models::user::{User, NewUser};
use std::error::Error;
use bigdecimal::{BigDecimal, FromPrimitive};
use crate::errors::DiscordIdConversionError;

pub fn load_users(conn: &PgConnection) -> Result<Vec<User>, Box<dyn Error>>  {
    use crate::schema::users::dsl::*;

    Ok(users.load::<User>(conn)?)
}

pub fn load_user(conn: &PgConnection, user_name: &str) -> Result<User, Box<dyn Error>> {
    use crate::schema::users::dsl::*;

    Ok(users.filter(name.eq(user_name))
        .first(conn)?)
}

pub fn load_user_by_discord_id(conn: &PgConnection, discord_id_number: &u64) -> Result<User, Box<dyn Error>> {
    use crate::schema::users::dsl::*;

    let numeric_discord_id = BigDecimal::from_u64(*discord_id_number)
        .ok_or_else(|| { DiscordIdConversionError::new() })?;

    Ok(users.filter(discord_id.eq(numeric_discord_id))
        .first(conn)?)
}

pub fn create_user(conn: &PgConnection, name: &str, discord_id: &u64) -> Result<User, Box<dyn Error>> {
    use crate::schema::users;

    let numeric_discord_id = BigDecimal::from_u64(*discord_id)
        .ok_or_else(|| { DiscordIdConversionError::new() })?;

    let new_user = NewUser::new(name, numeric_discord_id);

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

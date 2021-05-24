use crate::file_io::data::location_of;
use crate::models::users::*;

use std::fs;
use std::io::Error;

pub fn register_user(user: User) -> Result<(), Error> {
    let mut users = load_users()?;
    if users.users.contains(&user) {
        return Ok(())
    }
    users.users.push(user);
    save_users(users)?;
    Ok(())
}

fn load_users() -> Result<UsersList, Error> {
    let users_location = location_of("data/users.json");
    let file = fs::read_to_string(users_location)?;

    let user_list = serde_json::from_str(&file)?;
    Ok(user_list)
}

fn save_users(user_list: UsersList) -> Result<(), Error> {
    let data_to_save = serde_json::to_string(&user_list)?;
    let users_location = location_of("data/users.json");
    fs::write(users_location, data_to_save)?;
    Ok(())
}

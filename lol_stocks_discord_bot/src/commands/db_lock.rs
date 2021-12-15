use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

use std::error::Error;
use std::result::Result;

use lol_stocks_core::database::{
    connection::establish_connection,
    locks::{lock_database, unlock_database, load_lock},
    users::load_user,
};

#[command]
pub async fn db_lock(ctx: &Context, msg: &Message) -> CommandResult {
    let user_name = &msg.author.name;
    let response: String;
    match turn_key(&user_name) {
        Ok(message) => { response = message  },
        Err(e) => { response = format!("An error occurred: {}", e.to_string()); }
    }
    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}

fn turn_key(user_name: &str) -> Result<String, Box<dyn Error>> {
    let conn = establish_connection();
    let user = load_user(&conn, user_name)?;
    let db_lock = load_lock(&conn)?;
    if user.admin {
        if db_lock.locked {
            println!("Unlocking database");
            match unlock_database(&conn) {
                Ok(_) => Ok("Market is open! Happy Shopping".to_string()),
                Err(e) => Err(e)
            }
        } else {
            println!("Locking database");
            match lock_database(&conn) {
                Ok(_) => Ok("Market is closed! Time to watch some games".to_string()),
                Err(e) => Err(e)
            }
        }
    } else {
        Ok("Only admins can do this!".to_string())
    }
}

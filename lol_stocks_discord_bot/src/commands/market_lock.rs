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
    users::load_user_by_discord_id,
};
use crate::helpers::messages;

#[command]
pub async fn market_lock(ctx: &Context, msg: &Message) -> CommandResult {
    let user_discord_id = msg.author.id.as_u64();
    let mut response: Option<String> = None;
    let mut error_message: Option<String> = None;

    match turn_key(user_discord_id) {
        Ok(message) => { response = Some(message)  },
        Err(e) => { error_message = Some(e.to_string()) }
    }

    if error_message.is_some() {
        messages::send_error_message(ctx, msg, error_message.unwrap()).await?;
    }

    if response.is_some() {
        messages::send_message::<String, &str>(
            ctx,
            msg,
            response.unwrap(),
            None,
            None
        ).await?;
    }
    Ok(())
}

fn turn_key(user_discord_id: &u64) -> Result<String, Box<dyn Error>> {
    let conn = establish_connection();
    let user = load_user_by_discord_id(&conn, user_discord_id)?;
    let db_lock = load_lock(&conn)?;
    if user.admin {
        if db_lock.locked {
            println!("Unlocking database");
            match unlock_database(&conn) {
                Ok(_) => Ok("🔓 Market is open! Happy Shopping".to_string()),
                Err(e) => Err(e)
            }
        } else {
            println!("Locking database");
            match lock_database(&conn) {
                Ok(_) => Ok("🔒 Market is closed! Time to watch some games".to_string()),
                Err(e) => Err(e)
            }
        }
    } else {
        Ok("Only admins can do this!".to_string())
    }
}

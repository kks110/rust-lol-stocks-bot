use std::error::Error;
use std::result::Result;

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

use lol_stocks_core::database::{
    connection::establish_connection,
    users::create_user
};
use lol_stocks_core::models::user::User;
use crate::helpers::messages;


#[command]
pub async fn register(ctx: &Context, msg: &Message) -> CommandResult {
    let mut response: Option<String> = None;
    let mut error_message: Option<String> = None;

    match create_new_user(&msg.author.name, msg.author.id.as_u64()) {
        Ok(user) => {
            response = Some(format!("💹 Created user {}. Starting Balance is {}", user.name, user.balance));
        },
        Err(e) => { error_message = Some(e.to_string()) }

    }

    if error_message.is_some() {
        messages::send_error_message(ctx, msg, error_message.unwrap()).await?;
    }

    if response.is_some() {
        messages::send_message::<String, String>(
            ctx,
            msg,
            response.unwrap(),
            None,
            None
        ).await?;
    }

    Ok(())
}

fn create_new_user(username: &str, discord_id: &u64) -> Result<User, Box<dyn Error>> {
    let conn = establish_connection();
    create_user(&conn, username, discord_id)
}

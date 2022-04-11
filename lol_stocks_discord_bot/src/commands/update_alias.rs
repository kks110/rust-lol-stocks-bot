use std::error::Error;
use std::result::Result;

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
    Args,
};

use lol_stocks_core::database::{
    connection::establish_connection,
    users::add_user_alias,
    users::load_user_by_discord_id,
};
use lol_stocks_core::models::user::User;
use crate::helpers::{messages, parse_args};


#[command]
pub async fn update_alias(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut response: Option<String> = None;
    let mut error_message: Option<String> = None;
    let mut alias: Option<String> = None;
    let mut user: Option<User> = None;

    match parse_args::parse_string(args) {
        Ok(a) => { alias = Some(a) },
        Err(e) => { error_message = Some(e) }
    };

    let conn = establish_connection();
    let user_discord_id = msg.author.id.as_u64();
    match load_user_by_discord_id(&conn, user_discord_id) {
        Ok(u) => { user = Some(u) }
        Err(e) => { error_message = Some(e.to_string()) }
    }

    if alias.is_some() && user.is_some() {
        match add_user_alias(&conn, &user.unwrap().name, &alias.unwrap()) {
            Ok(u) => { response = Some(format!("Alias updated for {}. Alias is: {}", u.name, u.alias.unwrap()))}
            Err(e) => { error_message = Some(e.to_string()) }
        }
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

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


#[command]
pub async fn register(ctx: &Context, msg: &Message) -> CommandResult {
    let response: String;

    match create_new_user(&msg.author.name, msg.author.id.0) {
        Ok(user) => {
            println!("{} has registered", user.name);
            response = format!("Updated user {}. Starting Balance is {}", user.name, user.balance);
        },
        Err(e) => {
            println!("There was an error creating the new user: {}", e.to_string());
            response = format!("There was an error creating the new user: {}", e.to_string());
        }
    }
    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}

fn create_new_user(username: &str, discord_id: u64) -> Result<User, Box<dyn Error>> {
    let conn = establish_connection();
    create_user(&conn, username, discord_id)
}

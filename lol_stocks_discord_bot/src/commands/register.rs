use std::error::Error;
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

    match create_new_user(&msg.author.name) {
        Ok(user) => {
            println!("{} has registered", user.name);
            let response = format!("Updated user {}. Starting Balance is {}", user.name, user.balance);
            msg.channel_id.say(&ctx.http, response).await?;
        },
        Err(e) => {
            println!("There was an error creating the new user: {}", e.to_string());
            let response = format!("There was an error creating the new user: {}", e.to_string());
            msg.channel_id.say(&ctx.http, response).await?;
        }
    }
    Ok(())
}

fn create_new_user(username: &str) -> Result<User, Box<dyn Error>> {
    let conn = establish_connection();
    Ok(create_user(&conn, username)?)
}

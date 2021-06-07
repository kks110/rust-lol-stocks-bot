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

#[command]
pub async fn register(ctx: &Context, msg: &Message) -> CommandResult {
    let conn = establish_connection();
    let new_user = create_user(&conn, &msg.author.name);

    let response = format!("Updated user {}. Starting Balance is {}", new_user.name, new_user.balance);
    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}

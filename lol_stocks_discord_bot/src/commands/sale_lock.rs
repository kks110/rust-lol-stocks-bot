use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{CommandResult, macros::command};

use lol_stocks_core::database::{
    connection::establish_connection,
    locks::lock_database,
};

#[command]
pub async fn sale_lock(ctx: &Context, msg: &Message) -> CommandResult {
    let conn = establish_connection();
    lock_database(&conn);

    let response = format!("Sales are locked");

    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}

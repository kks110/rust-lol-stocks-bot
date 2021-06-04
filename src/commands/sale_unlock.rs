use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{CommandResult, macros::command};

use crate::database::{
    connection::establish_connection,
    locks::unlock_database,
};

#[command]
pub async fn sale_unlock(ctx: &Context, msg: &Message) -> CommandResult {
    let conn = establish_connection();
    unlock_database(&conn);

    let response = format!("Sales are unlocked");

    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}

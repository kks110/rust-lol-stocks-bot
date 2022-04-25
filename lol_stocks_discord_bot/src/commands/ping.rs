use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};
use crate::helpers::messages;

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    messages::send_message::<&str, &str>(
        ctx,
        msg,
        "🏓 Pong",
        None,
        None
    ).await?;
    Ok(())
}

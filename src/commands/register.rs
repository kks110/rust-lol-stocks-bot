use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

#[command]
pub async fn register(ctx: &Context, msg: &Message) -> CommandResult {
    let name = &msg.author.name;

    msg.channel_id.say(&ctx.http, name).await?;

    Ok(())
}

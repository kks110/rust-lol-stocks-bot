use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e
                .colour(0x4287f5)
                .title("🏓 Pong")
        })
    }).await?;
    Ok(())
}

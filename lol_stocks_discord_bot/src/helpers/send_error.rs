use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

pub async fn send_error(ctx: &Context, msg: &Message, error: String) -> CommandResult {
    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e
                .colour(0xff5733)
                .title(format!("❌ An error as occurred: {}", error))
        })
    }).await?;
    Ok(())
}

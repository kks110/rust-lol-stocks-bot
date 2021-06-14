use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

#[command]
pub async fn schedule(ctx: &Context, msg: &Message) -> CommandResult {

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| e
            .colour(0x00ff00)
            .title(format!("Schedule Links:"))
            .field("LEC:", "<https://lolesports.com/schedule?leagues=lec>", false)
            .field("LCS:", "<https://lolesports.com/schedule?leagues=lcs>", false)
            .field("Both:", "<https://lolesports.com/schedule?leagues=lec,lcs>", false)
        )
    }).await?;

    Ok(())
}

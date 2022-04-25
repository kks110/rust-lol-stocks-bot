use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};
use crate::helpers::messages;

#[command]
pub async fn schedule(ctx: &Context, msg: &Message) -> CommandResult {
    let fields = vec![
        ("LEC:", "<https://lolesports.com/schedule?leagues=lec>", false),
        ("LCS:", "<https://lolesports.com/schedule?leagues=lcs>", false),
        ("Both:", "<https://lolesports.com/schedule?leagues=lec,lcs>", false)
    ];

    messages::send_message(
        ctx,
        msg,
        "Schedule Links",
        None,
        Some(fields)
    ).await?;

    Ok(())
}

use crate::models::users::User;

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

#[command]
pub async fn register(ctx: &Context, msg: &Message) -> CommandResult {
    let user = User {
        username: msg.author.name.to_owned(),
        balance: 2000.to_owned(),
        portfolio: Vec::new(),
    };

    let j = serde_json::to_string(&user)?;

    msg.channel_id.say(&ctx.http, j).await?;

    Ok(())
}

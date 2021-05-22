use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

const HELP_MESSAGE: &str = "
Hello there, Human!

You have summoned me. Let's see about getting you what you need.

? Need technical help?
=> Post in the <#845434690416148491> channel and other humans will assist you.

? Looking for the Code of Conduct?
=> Here it is: <https://opensource.facebook.com/code-of-conduct>

? Something wrong?
=> You can flag an admin with @admin

I hope that resolves your issue!
-- Helpbot

";


#[command]
pub async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, HELP_MESSAGE).await?;

    Ok(())
}
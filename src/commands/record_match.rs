// use crate::file_io::teams::update_elo;
//
// use serenity::prelude::*;
// use serenity::model::prelude::*;
// use serenity::framework::standard::{CommandResult, macros::command, Args};
//
// #[command]
// pub async fn record_match(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
//     let winner = args.single::<String>()?;
//     let loser = args.single::<String>()?;
//
//     update_elo(&winner, &loser)?;
//
//     msg.channel_id.say(&ctx.http, "matches recorded").await?;
//     Ok(())
// }

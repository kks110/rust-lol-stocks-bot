use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
    Args,
};

use crate::helpers::portfolio_view;

#[command]
pub async fn view_portfolio(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let user_name = match args.single::<String>() {
        Ok(user) => user,
        Err(_) => msg.author.name.clone()
    };

    let response: String;
    let user = portfolio_view::PlayerIdentification::PlayerName(user_name);

    match portfolio_view::make_portfolio_view(user) {
        Ok(message) => { response = message },
        Err(e) => { response = format!("An error has occurred: {}", e.to_string())}
    }

    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}

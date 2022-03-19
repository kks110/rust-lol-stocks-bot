use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
    Args,
};

use crate::helpers::{messages, portfolio_view};
use crate::helpers::portfolio_view::PlayersHoldings;

#[command]
pub async fn portfolio(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let user_name = match args.single::<String>() {
        Ok(user) => user,
        Err(_) => msg.author.name.clone()
    };

    let mut holdings: Option<PlayersHoldings> = None;
    let mut error_message: Option<String> = None;

    let user = portfolio_view::PlayerIdentification::PlayerName(user_name);

    match portfolio_view::list_holdings_for_player(user) {
        Ok(h) => { holdings = Some(h) },
        Err(e) => {  error_message = Some(e.to_string()) }
    }

    if error_message.is_some() {
        messages::send_error_message(ctx, msg, error_message.unwrap()).await?;
    }

    if holdings.is_some() {
        let h = holdings.unwrap();
        let mut response = "".to_string();
        response.push_str(&format!("**Balance:** {}\n", h.balance));

        response.push_str("──────────\n");

        for holding in h.holdings {
            let mut body: String = "".to_string();
            body.push_str(&format!("**{}:** {} ({})\n", holding.team.name, holding.amount, holding.value));
            response.push_str(&body);
        };

        response.push_str("──────────\n");
        response.push_str(&format!("**Total:** {}", h.total_value));

        messages::send_message::<String, String>(
            ctx,
            msg,
            format!("{}'s Portfolio:", h.user),
            Some(response),
            None
        ).await?;
    }

    Ok(())
}

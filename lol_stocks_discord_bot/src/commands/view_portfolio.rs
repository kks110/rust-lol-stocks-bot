use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
    Args,
};

use crate::helpers::portfolio_view;
use crate::helpers::portfolio_view::PlayersHoldings;
use crate::helpers::send_error::send_error;

#[command]
pub async fn view_portfolio(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let user_name = match args.single::<String>() {
        Ok(user) => user,
        Err(_) => msg.author.name.clone()
    };

    let mut holdings: PlayersHoldings = PlayersHoldings{
        holdings: vec![],
        user: "".to_string(),
        balance: 0,
        total_value: 0
    };
    let mut error_occurred: Option<String> = None;

    let user = portfolio_view::PlayerIdentification::PlayerName(user_name);

    match portfolio_view::list_holdings_for_player(user) {
        Ok(h) => { holdings = h },
        Err(e) => {  error_occurred = Some(e.to_string()) }
    }

    if error_occurred.is_some() {
        send_error(ctx, msg, error_occurred.unwrap()).await?;
        return Ok(())
    }

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            let mut response = "".to_string();
            response.push_str(&format!("**Balance:** {}\n", holdings.balance));

            response.push_str("──────────\n");

            for holding in holdings.holdings {
                let mut body: String = "".to_string();
                body.push_str(&format!("**{}:** {} ({})\n", holding.team.name, holding.amount, holding.value));
                response.push_str(&body);
            };

            response.push_str("──────────\n");
            response.push_str(&format!("**Total:** {}", holdings.total_value));

            e
                .colour(0x4287f5)
                .title(format!("{}'s Portfolio:", holdings.user))
                .description(response)
        })
    }).await?;
    Ok(())
}

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
    Args,
};

use std::error::Error;
use std::result::Result;

use lol_stocks_core::database::{
    connection::establish_connection,
    portfolios::user_portfolio_purchase,
    locks::load_lock,
    users::{load_user_by_discord_id, update_user},
    teams::load_team,
};
use crate::helpers::portfolio_view;
use crate::helpers::portfolio_view::PlayersHoldings;

#[command]
pub async fn buy_all(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let user_discord_id = msg.author.id.as_u64();

    let mut response: String;

    match parse_args(args) {
        Ok(t) => {
            let team_name = t;
            match perform_buy_all(&team_name, user_discord_id) {
                Ok(message) => { response = message },
                Err(e) => { response = format!("An error has occurred: {}", e)}
            }
        },
        Err(e) => { response = format!("An error as occurred {}", e.to_string()); }
    }

    let mut holdings: PlayersHoldings = PlayersHoldings{
        holdings: vec![],
        user: "".to_string(),
        balance: 0,
        total_value: 0
    };
    let mut error_occurred: Option<String> = None;

    let user = portfolio_view::PlayerIdentification::PlayerId(*user_discord_id);

    match portfolio_view::list_holdings_for_player(user) {
        Ok(h) => { holdings = h },
        Err(e) => {  error_occurred = Some(e.to_string()) }
    }

    if error_occurred.is_some() {
        msg.channel_id.say(
            &ctx.http,
            format!("An Error as occurred: {}", error_occurred.unwrap().to_string())
        ).await?;
        return Ok(())
    }

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e
                .colour(0x4287f5)
                .title(response)
        })
    }).await?;

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

fn parse_args(mut args: Args) -> Result<String, Box<dyn Error>> {
    Ok(args.single::<String>()?)
}

fn perform_buy_all(team_name: &str, user_discord_id: &u64) -> Result<String, Box<dyn Error>> {
    let conn = establish_connection();
    let db_lock = load_lock(&conn)?;

    if db_lock.locked {
        return Ok("🔒 Market is closed".to_string())
    }

    let team = load_team(&conn, team_name)?;
    let user = load_user_by_discord_id(&conn, user_discord_id)?;

    let amount = user.balance / team.elo;
    if amount == 0 {
        return Ok("❌ Not enough funds!".to_string())
    }

    let cost = team.elo * amount;
    update_user(&conn, &user.name, user.balance - cost)?;
    user_portfolio_purchase(&conn, &user, &team, amount)?;
    Ok("💸 Purchase Made!".to_string())
}
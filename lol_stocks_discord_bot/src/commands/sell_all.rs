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
    users::{load_user_by_discord_id, update_user},
    teams::{load_team_by_id, load_team},
    portfolios::{load_users_portfolio, user_portfolio_sell},
    locks::load_lock,
};
use lol_stocks_core::portfolio_calculations::calculate_portfolio_value;
use crate::helpers::portfolio_view;
use crate::helpers::portfolio_view::PlayersHoldings;

#[command]
pub async fn sell_all(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let user_discord_id = msg.author.id.as_u64();
    let team_name: Option<String>;

    match args.single::<String>() {
        Ok(team) => team_name = Some(team),
        Err(_) => team_name = None
    }

    let mut response: String;

    match perform_sell_all(team_name, user_discord_id) {
        Ok(message) => { response = message }
        Err(e) => { response = format!("An error has occurred: {}", e) }
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

fn perform_sell_all(team_name: Option<String>, user_discord_id: &u64) -> Result<String, Box<dyn Error>> {
    let conn = establish_connection();
    let db_lock = load_lock(&conn)?;

    if db_lock.locked {
        return Ok("🔒 Market is closed".to_string());
    }

    let user = load_user_by_discord_id(&conn, user_discord_id)?;
    let users_portfolio = load_users_portfolio(&conn, &user)?;

    if team_name.is_some() {
        let team = load_team(&conn, &team_name.unwrap())?;
        for portfolio in users_portfolio {
            if portfolio.team_id == team.id {
                let new_balance = team.elo * portfolio.amount + user.balance;
                update_user(&conn, &user.name, new_balance)?;
                user_portfolio_sell(&conn, &user, &team, portfolio.amount)?;
                return Ok("💸 Sale Made!".to_string());
            }
        }
    } else {
        let new_balance = calculate_portfolio_value(&conn, &user, &users_portfolio)?;
        update_user(&conn, &user.name, new_balance)?;
        for portfolio in users_portfolio {
            let team = load_team_by_id(&conn, &portfolio.team_id)?;
            user_portfolio_sell(&conn, &user, &team, portfolio.amount)?;
        }
        return Ok("💸 Full portfolio sold".to_string());
    }
    Ok("❌ You do not own those shares".to_string())
}

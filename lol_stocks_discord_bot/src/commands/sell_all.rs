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
use crate::helpers::{messages, portfolio_view};

#[command]
pub async fn sell_all(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let user_discord_id = msg.author.id.as_u64();
    let team_name: Option<String>;

    match args.single::<String>() {
        Ok(team) => team_name = Some(team),
        Err(_) => team_name = None
    }

    let mut title: Option<String> = None;
    let mut error_message: Option<String> = None;

    match perform_sell_all(team_name, user_discord_id) {
        Ok(message) => { title = Some(message) }
        Err(e) => { error_message = Some(e.to_string()) }
    }

    let mut holdings: Option<portfolio_view::PlayersHoldings> = None;
    let user = portfolio_view::PlayerIdentification::PlayerId(*user_discord_id);

    match portfolio_view::list_holdings_for_player(user) {
        Ok(h) => { holdings = Some(h) },
        Err(e) => {  error_message = Some(e.to_string()) }
    }

    if error_message.is_some() {
        messages::send_error_message(ctx, msg, error_message.unwrap()).await?;
    }

    if title.is_some() {
        messages::send_message::<String, &str>(
            ctx,
            msg,
            title.unwrap(),
            None,
            None
        ).await?;
    }

    if holdings.is_some() {
        messages::send_portfolio(ctx, msg, holdings.unwrap()).await?;
    }

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

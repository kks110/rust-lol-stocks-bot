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
    users::{load_user, update_user},
    teams::load_team,
};

#[command]
pub async fn buy_all(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let user_name = msg.author.name.clone();

    let response: String;

    match parse_args(args) {
        Ok(t) => {
            let team_name = t;
            match perform_buy_all(&team_name, &user_name) {
                Ok(message) => { response = message },
                Err(e) => { response = format!("An error has occurred: {}", e)}
            }
        },
        Err(e) => { response = format!("An error as occurred {}", e.to_string()); }
    }

    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}

fn parse_args(mut args: Args) -> Result<String, Box<dyn Error>> {
    Ok(args.single::<String>()?)
}

fn perform_buy_all(team_name: &str, user_name: &str) -> Result<String, Box<dyn Error>> {
    let conn = establish_connection();
    let db_lock = load_lock(&conn)?;

    if db_lock.locked {
        return Ok("Sales are locked, wait for the games to finish!".to_string())
    }

    let team = load_team(&conn, team_name)?;
    let user = load_user(&conn, user_name)?;

    let amount = user.balance / team.elo;
    if amount == 0 {
        return Ok("Not enough funds!".to_string())
    }

    let cost = team.elo * amount;
    update_user(&conn, &user.name, user.balance - cost)?;
    user_portfolio_purchase(&conn, &user, &team, amount)?;
    Ok("Purchase Made!".to_string())
}
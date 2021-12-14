use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
    Args,
};

use std::error::Error;

use lol_stocks_core::database::{
    connection::establish_connection,
    users::{load_user, update_user},
    teams::load_team,
    portfolios::{load_users_portfolio, user_portfolio_sell},
    locks::load_lock,
};
use lol_stocks_core::models::{
    team::Team,
    user::User
};

#[command]
pub async fn sell(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let team_name = args.single::<String>()?;
    let amount = args.single::<i32>()?;
    let user_name = msg.author.name.clone();

    let response: String;

    match sell_shares(amount, &team_name, &user_name) {
        Ok(message) => {
            println!("{} and purchased {} shares in {}", user_name, amount, team_name);
            response = message
        },
        Err(e) => {
            response = format!("An error as occurred {}", e.to_string());
            println!("{}", response);
        }
    }

    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}

fn sell_shares(amount: i32, team_name: &str, user_name: &str) -> Result<String, Box<dyn Error>> {
    let conn = establish_connection();

    let db_lock = load_lock(&conn)?;

    if db_lock.locked {
        return Ok("Market is closed".to_string())
    }

    if amount <= 0 {
        return Ok("Please enter a positive number!".to_string())
    }

    let team = load_team(&conn, team_name)?;
    let user = load_user(&conn, &user_name)?;
    let users_portfolio = load_users_portfolio(&conn, &user)?;

    let new_balance = team.elo * amount + user.balance;

    for portfolio in users_portfolio {
        if portfolio.team_id == team.id {
            return if amount <= portfolio.amount {
                match update_portfolio(new_balance, &user, &team, amount) {
                    Ok(s) => Ok(s),
                    Err(e) => Err(e)
                }
            } else {
                Ok("You don't have that many shares".to_string())
            }
        }
    }

    Ok("You don't own those shares".to_string())
}

fn update_portfolio(new_balance: i32, user: &User, team: &Team, amount: i32) -> Result<String, Box<dyn Error>> {
    let conn = establish_connection();
    update_user(&conn, &user.name, new_balance)?;
    user_portfolio_sell(&conn,&user, &team, amount)?;
    Ok("Sale Made!".to_string())
}
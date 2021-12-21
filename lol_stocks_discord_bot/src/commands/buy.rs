use std::error::Error;
use std::result::Result;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
    Args,
};

use lol_stocks_core::database::{
    connection::establish_connection,
    portfolios::user_portfolio_purchase,
    locks::load_lock,
    users::{load_user, update_user},
    teams::load_team,
};
use lol_stocks_core::models::{
    team::Team,
    user::User
};

#[command]
pub async fn buy(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let response: String;

    match parse_args(args) {
        Ok(team_and_amount) => {
            let (team_name, amount) = team_and_amount;
            let user_name = msg.author.name.clone();

            match buy_shares(amount, &team_name, &user_name) {
                Ok(message) => {
                    println!("{} and purchased {} shares in {}", user_name, amount, team_name);
                    response = message;
                },
                Err(e) => {
                    response = format!("An error as occurred {}", e.to_string());
                    println!("{}", response);
                }
            }
        },
        Err(e) => {
            response = format!("An error as occurred {}", e.to_string());
        }
    }

    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}

fn parse_args(mut args: Args) -> Result<(String, i32), Box<dyn Error>> {
    Ok((args.single::<String>()?, args.single::<i32>()?))
}

fn buy_shares(amount: i32, team_name: &str, user_name: &str) -> Result<String, Box<dyn Error>> {
    let conn = establish_connection();

    let db_lock = load_lock(&conn)?;

    if db_lock.locked {
        return Ok("Market is closed".to_string())
    }

    if amount <= 0 {
        return Ok("Please enter a positive number!".to_string())
    }

    let team = load_team(&conn, team_name)?;
    let user = load_user(&conn, user_name)?;

    if amount > user.balance {
        return Ok("Not enough funds".to_string())
    }

    let cost: i32 = team.elo * amount;

    if cost <= user.balance {
        return match update_balance(cost, user, team, amount) {
            Ok(s) => Ok(s),
            Err(e) => Err(e)
        }
    } else {
        Ok("Not enough funds".to_string())
    }
}

fn update_balance(cost: i32, user: User, team: Team, amount: i32) -> Result<String, Box<dyn Error>> {
    let conn = establish_connection();
    update_user(&conn, &user.name, user.balance - cost)?;
    user_portfolio_purchase(&conn, &user, &team, amount)?;
    Ok("Purchase Made!".to_string())
}

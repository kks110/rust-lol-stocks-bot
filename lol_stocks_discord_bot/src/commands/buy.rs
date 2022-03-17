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
    users::{load_user_by_discord_id, update_user},
    teams::load_team,
};

use lol_stocks_core::models::{
    team::Team,
    user::User
};
use crate::helpers::portfolio_view;

#[command]
pub async fn buy(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut response: String;
    let user_discord_id = msg.author.id.as_u64();

    match parse_args(args) {
        Ok(amount_and_team) => {
            let (amount, team_name) = amount_and_team;

            match buy_shares(amount, &team_name, user_discord_id) {
                Ok(message) => {
                    response = message;
                },
                Err(e) => {
                    response = format!("An error as occurred {}", e.to_string());
                }
            }
        },
        Err(e) => {
            response = format!("An error as occurred {}", e.to_string());
        }
    }

    // let user = portfolio_view::PlayerIdentification::PlayerId(*user_discord_id);
    //
    // match portfolio_view::make_portfolio_view(user) {
    //     Ok(message) => { response.push_str(&format!("\n\n{}", message)) },
    //     Err(e) => { response = format!("An error has occurred: {}", e.to_string())}
    // }

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e
                .colour(0x4287f5)
                .title(response)
        })
    }).await?;
    Ok(())
}

fn parse_args(mut args: Args) -> Result<(i32, String), Box<dyn Error>> {
    Ok((args.single::<i32>()?, args.single::<String>()?))
}

fn buy_shares(amount: i32, team_name: &str, user_discord_id: &u64) -> Result<String, Box<dyn Error>> {
    let conn = establish_connection();

    let db_lock = load_lock(&conn)?;

    if db_lock.locked {
        return Ok("🔒 Market is closed".to_string())
    }

    if amount <= 0 {
        return Ok("❌ Please enter a positive number!".to_string())
    }

    let team = load_team(&conn, team_name)?;
    let user = load_user_by_discord_id(&conn, user_discord_id)?;

    if amount > user.balance {
        return Ok("❌ Not enough funds".to_string())
    }

    let cost: i32 = team.elo * amount;

    if cost <= user.balance {
        return match update_balance(cost, user, team, amount) {
            Ok(s) => Ok(s),
            Err(e) => Err(e)
        }
    } else {
        Ok("❌ Not enough funds".to_string())
    }
}

fn update_balance(cost: i32, user: User, team: Team, amount: i32) -> Result<String, Box<dyn Error>> {
    let conn = establish_connection();
    update_user(&conn, &user.name, user.balance - cost)?;
    user_portfolio_purchase(&conn, &user, &team, amount)?;
    Ok("💸 Purchase Made!".to_string())
}

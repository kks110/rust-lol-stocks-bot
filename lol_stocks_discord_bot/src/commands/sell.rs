use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
    Args,
};

use lol_stocks_core::database::{
    connection::establish_connection,
    users::{load_user, update_user},
    teams::load_team,
    portfolios::{load_users_portfolio, user_portfolio_sell},
    locks::load_lock,
};
use lol_stocks_core::models::lock::Lock;

#[command]
pub async fn sell(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let team_name = args.single::<String>()?;
    let amount = args.single::<i32>()?;
    let user_name = msg.author.name.clone();
    let conn = establish_connection();
    let db_lock: Option<Lock>;
    let mut response= String::from("");

    match load_lock(&conn) {
        Ok(l) => db_lock = Some(l),
        Err(e) => {
            response.push_str(&e);
            db_lock = None
        }
    }

    if db_lock.is_some() {
        if amount <= 0 {
            response = format!("Please enter a positive number!");
        } else {
            if db_lock.unwrap().locked {
                response = format!("Sales are locked, wait for the games to finish!");
            } else {
                let team = load_team(&conn, &team_name);
                let user = load_user(&conn, &user_name);
                response = format!("You don't own these shares");

                let users_portfolio = load_users_portfolio(&conn, &user);

                let new_balance = team.elo * amount + user.balance;

                for portfolio in users_portfolio {
                    if portfolio.team_id == team.id {
                        response = format!("You don't have that many shares");
                        if amount <= portfolio.amount {
                            update_user(&conn, &user.name, new_balance);
                            user_portfolio_sell(&conn,&user, &team, amount);
                            response = format!("Sale Made!");
                        }
                    }
                }
            }
        }
    }

    println!("{} and sold {} shares in {}", user_name, amount, team_name);
    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}

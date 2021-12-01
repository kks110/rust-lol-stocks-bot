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
    teams::{load_team_by_id, load_team},
    portfolios::{load_users_portfolio, user_portfolio_sell},
    locks::load_lock,
};
use lol_stocks_core::models::lock::Lock;
use lol_stocks_core::portfolio_calculations::calculate_portfolio_value;

#[command]
pub async fn sell_all(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
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
        if db_lock.unwrap().locked {
            response = format!("Sales are locked, wait for the games to finish!");
        } else {
            let user = load_user(&conn, &user_name);
            let users_portfolio = load_users_portfolio(&conn, &user);

            match args.single::<String>() {
                Ok(team_name) => {
                    let team = load_team(&conn, &team_name);

                    for portfolio in users_portfolio {
                        if portfolio.team_id == team.id {
                            let new_balance = team.elo * portfolio.amount + user.balance;
                            update_user(&conn, &user.name, new_balance);
                            user_portfolio_sell(&conn,&user, &team, portfolio.amount);
                        }
                    }
                }
                Err(_) => {
                    let new_balance = calculate_portfolio_value(&conn, &user, &users_portfolio);
                    update_user(&conn, &user.name, new_balance);
                    for portfolio in users_portfolio {
                        let team = load_team_by_id(&conn, &portfolio.team_id);
                        user_portfolio_sell(&conn, &user, &team, portfolio.amount);
                    }
                }
            }
            response = format!("Sale Made!");
        }
    }

    println!("{} sold all their portfolio", user_name);
    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}

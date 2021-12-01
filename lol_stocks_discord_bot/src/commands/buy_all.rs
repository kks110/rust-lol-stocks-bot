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
use lol_stocks_core::models::lock::Lock;

#[command]
pub async fn buy_all(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let team_name = args.single::<String>()?;
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
            let team = load_team(&conn, &team_name);
            let user = load_user(&conn, &user_name);

            let amount = user.balance / team.elo;
            if amount == 0 {
                response = format!("Not enough funds!");
            } else {
                let cost = team.elo * amount;
                update_user(&conn, &user.name, user.balance - cost);
                user_portfolio_purchase(&conn, &user, &team, amount);
                response = format!("Purchase Made!");
            }
        }
    }

    println!("{} and purchased shares in {}", user_name, team_name);
    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}

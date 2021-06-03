use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
    Args,
};

use crate::database::connection::establish_connection;
use crate::database::users::load_user;
use crate::database::users::update_user;
use crate::database::teams::load_team;
use crate::database::portfolios::user_portfolio_purchase;

#[command]
pub async fn buy(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let team_name = args.single::<String>()?;
    let amount = args.single::<i32>()?;
    let user_name = msg.author.name.clone();
    let conn = establish_connection();

    let team = load_team(&conn, &team_name);
    let user = load_user(&conn, &user_name);
    let mut response = format!("Not enough funds!");
    let cost: i32 = team.elo * amount;
    if cost <= user.balance {
        update_user(&conn, &user.name, user.balance - cost);
        user_portfolio_purchase(&conn,&user, &team, amount);
        response = format!("Purchase Made!");
    }

    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{CommandResult, macros::command};

use crate::database::{
    connection::establish_connection,
    users::load_user,
    portfolios::load_users_portfolio,
    teams::load_team_by_id
};


#[command]
pub async fn view_portfolio(ctx: &Context, msg: &Message) -> CommandResult {
    let conn = establish_connection();
    let user_name = msg.author.name.clone();

    let user = load_user(&conn, &user_name);
    let portfolio = load_users_portfolio(&conn, &user);

    let mut response: String = String::from("");

    let mut value = 0;

    let user_balance = format!("User: {}, Balance: {}\n", user.name, user.balance);
    value = value + user.balance;
    response.push_str(&user_balance);

    for holding in portfolio {
        let team = load_team_by_id(&conn, &holding.team_id);
        let holding_value = team.elo * holding.amount;
        value = value + holding_value;
        let portfolio_line = format!("Team: {}, Amount: {}, Value: {}\n", team.name, holding.amount, holding_value);
        response.push_str(&portfolio_line);
    }

    let total_value = format!("Total Portfolio Value: {}", value);
    response.push_str(&total_value);

    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}

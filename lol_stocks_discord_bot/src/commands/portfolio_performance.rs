use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

use lol_stocks_core::{
    portfolio_calculations::calculate_portfolio_value,
    database::{
        connection::establish_connection,
        users::load_user,
        user_portfolio_histories::load_user_portfolio_history,
        portfolios::load_users_portfolio,
    }
};

#[command]
pub async fn portfolio_performance(ctx: &Context, msg: &Message) -> CommandResult {
    let user_name = msg.author.name.clone();
    let conn = establish_connection();
    let user = load_user(&conn, &user_name);
    let user_portfolio_history = load_user_portfolio_history(&conn, &user);

    let portfolio = load_users_portfolio(&conn, &user);
    let current_value = calculate_portfolio_value(&conn, &user, &portfolio);

    let mut response = format!("Date: Now, Value: {}\n", current_value);

    for entry in user_portfolio_history {
        let response_line = format!("Date: {}, Value: {}\n", entry.date, entry.value);
        response.push_str(&response_line)
    }

    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}

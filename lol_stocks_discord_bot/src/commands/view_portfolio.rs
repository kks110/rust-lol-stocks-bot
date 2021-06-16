use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{CommandResult, macros::command};

use lol_stocks_core::database::{
    connection::establish_connection,
    users::load_user,
    portfolios::load_users_portfolio,
    teams::load_team_by_id
};
use lol_stocks_core::models::team::Team;

struct Holding {
    pub team: Team,
    pub amount: i32,
    pub value: i32
}

#[command]
pub async fn view_portfolio(ctx: &Context, msg: &Message) -> CommandResult {
    let conn = establish_connection();
    let user_name = msg.author.name.clone();

    let user = load_user(&conn, &user_name);
    let portfolio = load_users_portfolio(&conn, &user);

    let mut holdings: Vec<Holding> = Vec::new();
    for item in portfolio {
        let team = load_team_by_id(&conn, &item.team_id);
        let value = team.elo * item.amount;
        holdings.push(Holding { team, value, amount: item.amount })
    }
    holdings.sort_by(|a, b| b.value.cmp(&a.value));

    let mut response: String = String::from("");

    let mut value = 0;

    let user_balance = format!("User: {}, Balance: {}\n", user.name, user.balance);
    value = value + user.balance;
    response.push_str(&user_balance);

    for holding in holdings {
        value = value + holding.value;
        let portfolio_line = format!("Team: {}, Amount: {}, Value: {}\n", holding.team.name, holding.amount, holding.value);
        response.push_str(&portfolio_line);
    }

    let total_value = format!("Total Portfolio Value: {}", value);
    response.push_str(&total_value);

    println!("{} has viewed their portfolio", user_name);
    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

use std::collections::HashMap;

use lol_stocks_core::database::{
    connection::establish_connection,
    portfolios::load_all_portfolios,
    teams::load_team_by_id,

};

#[command]
pub async fn market_cap(ctx: &Context, msg: &Message) -> CommandResult {
    let conn = establish_connection();

    let portfolios = load_all_portfolios(&conn);

    let mut amount_count: HashMap<String, i32> = HashMap::new();

    for portfolio in portfolios {
        let team = load_team_by_id(&conn, &portfolio.team_id);
        *amount_count.entry(team.name.to_owned()).or_insert(0) += portfolio.amount;
    }

    let mut amount_count_vec: Vec<(&String, &i32)> = amount_count.iter().collect();
    amount_count_vec.sort_by(|a, b| b.1.cmp(a.1));

    let mut response: String = String::from("");
    for (team, amount) in amount_count_vec {
        response.push_str(&format!("{}: {}\n", team, amount));
    }

    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}

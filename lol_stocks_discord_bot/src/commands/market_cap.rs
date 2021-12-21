use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

use std::error::Error;
use std::result::Result;

use std::collections::HashMap;

use lol_stocks_core::database::{
    connection::establish_connection,
    portfolios::load_all_portfolios,
    teams::load_team_by_id,

};

struct MarketCapEntry {
    pub team_name: String,
    pub amount: i32,
    pub value: i32
}

#[command]
pub async fn market_cap(ctx: &Context, msg: &Message) -> CommandResult {
    let response: String;

    match load_market_cap() {
        Ok(message) => { response = message},
        Err(e) => { response = format!("An Error Occurred: {}", e)}
    }

    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}

fn load_market_cap() -> Result<String, Box<dyn Error>> {
    let conn = establish_connection();

    let portfolios = load_all_portfolios(&conn)?;

    let mut amount_count: HashMap<i32, i32> = HashMap::new();

    for portfolio in portfolios {
        *amount_count.entry(portfolio.team_id).or_insert(0) += portfolio.amount;
    }

    let mut market_cap_entries: Vec<MarketCapEntry> = Vec::new();

    for (team_id, amount) in amount_count {
        let team = load_team_by_id(&conn, &team_id)?;

        market_cap_entries.push(MarketCapEntry{
            team_name: team.name.to_owned(),
            amount,
            value: amount * team.elo
        })
    }

    market_cap_entries.sort_by(|a, b| b.value.cmp(&a.value));

    let mut response: String = String::from("");
    for entry in market_cap_entries {
        response.push_str(&format!("{}: {} ({})\n", entry.team_name, entry.amount, entry.value));
    }
    Ok(response)
}

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
use lol_stocks_core::database::users::load_user_by_id;
use lol_stocks_core::models::portfolio::Portfolio;

struct OwnerEntry {
    pub name: String,
    pub amount: i32,
    pub value: i32
}

struct MarketCapEntry {
    pub team_name: String,
    pub amount: i32,
    pub value: i32,
    pub owners: Vec<OwnerEntry>
}

#[command]
pub async fn market_cap(ctx: &Context, msg: &Message) -> CommandResult {
    let mut entries: Vec<MarketCapEntry>  = vec![];
    let mut error_occurred: Option<String> = None;

    match load_market_cap() {
        Ok(message) => { entries = message},
        Err(e) => { error_occurred = Some(e.to_string())}
    }

    if error_occurred.is_some() {
        msg.channel_id.say(
            &ctx.http,
            format!("An Error as occurred: {}", error_occurred.unwrap().to_string())
        ).await?;
        return Ok(())
    }

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            let mut response = vec![];


            for entry in entries {
                let title = entry.team_name;
                let mut body: String = "".to_string();
                body.push_str(&format!("**All:** {} ({})\n", entry.amount, entry.value));
                body.push_str("──────────\n");
                for owner in entry.owners {
                    body.push_str(&format!("**{}:** {} ({})\n", owner.name, owner.amount, owner.value));
                }
                body.push_str("──────────\n");

                response.push((title, body, false))
            };
            e
                .colour(0x4287f5)
                .title("Market Cap:".to_string())
                .fields(response)
        })
    }).await?;
    Ok(())
}

fn load_market_cap() -> Result<Vec<MarketCapEntry>, Box<dyn Error>> {
    let conn = establish_connection();

    let portfolios = load_all_portfolios(&conn)?;

    let mut teams_and_owners: HashMap<i32, Vec<Portfolio>> = HashMap::new();

    for portfolio in portfolios {
        teams_and_owners.entry(portfolio.team_id).or_insert(vec![]).push(portfolio);
    }

    let mut market_cap_entries: Vec<MarketCapEntry> = Vec::new();

    for (team_id, owners) in teams_and_owners {
        let team = load_team_by_id(&conn, &team_id)?;

        let mut mc_entry = MarketCapEntry{
            team_name: team.name.to_string(),
            amount: 0,
            value: 0,
            owners: vec![]
        };

        for owner in owners {
            let player = load_user_by_id(&conn, &owner.user_id)?;
            let o = OwnerEntry {
                name: player.name.to_string(),
                amount: owner.amount,
                value: owner.amount * team.elo
            };
            mc_entry.amount += o.amount;
            mc_entry.value += o.value;
            mc_entry.owners.push(o);
        }

        mc_entry.owners.sort_by(|a, b| b.value.cmp(&a.value));

        market_cap_entries.push(mc_entry)
    }

    market_cap_entries.sort_by(|a, b| b.value.cmp(&a.value));

    Ok(market_cap_entries)
}

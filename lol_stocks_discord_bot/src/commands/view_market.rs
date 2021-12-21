use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
    Args,
};

use std::error::Error;
use std::result::Result;

use lol_stocks_core::models::team::Team;

use lol_stocks_core::database::{
    connection::establish_connection,
    teams::load_teams,
    teams::load_teams_by_league,
    leagues::load_league
};

#[command]
pub async fn view_market(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let response: String;
    let market: Option<String>;

    match args.single::<String>() {
        Ok(market_name) => market = Some(market_name),
        Err(_) => market = None
    }

    match make_view_market(market) {
        Ok(message) => { response = message },
        Err(e) => { response = format!("An error has occurred: {}", e.to_string())}
    }

    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}

fn make_view_market(market: Option<String>) -> Result<String, Box<dyn Error>> {
    let conn = establish_connection();

    let teams: Vec<Team>;

    if market.is_some() {
        let market_u = market.unwrap();
        load_league(&conn, &market_u)?;
        teams = load_teams_by_league(&conn, &market_u.to_uppercase())?
    } else {
        teams = load_teams(&conn)?
    }

    let mut message: String = "".to_string();
    for team in teams {
        let team_line = format!("{}  -  {}\n", team.name, team.elo);
        message.push_str(&team_line);
    }
    Ok(message)
}
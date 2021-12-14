use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
    Args,
};

use std::error::Error;

use lol_stocks_core::database::{
    connection::establish_connection,
    teams::load_teams,
    teams::load_teams_by_league,
    leagues::load_leagues,
};

#[command]
pub async fn view_market(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let market = args.single::<String>()?;

    let response: String;

    match make_view_market(&market) {
        Ok(message) => { response = message },
        Err(e) => { response = format!("An error has occured: {}", e.to_string())}
    }

    println!("Market displayed");
    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}

fn make_view_market(market: &str) -> Result<String, Box<dyn Error>> {
    let conn = establish_connection();

    let mut teams = load_teams(&conn)?;
    let leagues = load_leagues(&conn)?;

    for league in leagues {
        if league.name == market.to_uppercase() {
            teams = load_teams_by_league(&conn, &market.to_uppercase())?
        }
    }

    let mut message: String = "".to_string();
    for team in teams {
        let team_line = format!("{}  -  {}\n", team.name, team.elo);
        message.push_str(&team_line);
    }
    Ok(message)
}
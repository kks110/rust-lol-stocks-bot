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
use crate::helpers::messages;

#[command]
pub async fn market(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut teams: Option<Vec<Team>> = None;
    let market: Option<String>;
    let mut error_message: Option<String> = None;

    match args.single::<String>() {
        Ok(market_name) => market = Some(market_name.to_lowercase()),
        Err(_) => market = None
    }

    match make_view_market(&market) {
        Ok(t) => { teams = Some(t) },
        Err(e) => { error_message = Some(e.to_string())}
    }

    if error_message.is_some() {
        messages::send_error_message(ctx, msg, error_message.unwrap()).await?;
    }

    let market_name = market.unwrap_or("".to_string());

    let market_image: &str =
        if market_name == "lec" {
            "lec.png"
        } else if market_name == "lcs" {
            "lcs.png"
        } else {
            "both.png"
        };

    if teams.is_some() {
        let mut description: String = "".to_string();
        for team in teams.unwrap() {
            description.push_str(&format!("**{}:** {}\n", team.name, team.elo));
        }

        messages::send_image_as_attachment::<String, String>(
            ctx,
            msg,
            format!("{} market", market_name),
            Some(description),
            None,
            market_image
        ).await?;
    }

    Ok(())
}

fn make_view_market(market: &Option<String>) -> Result<Vec<Team>, Box<dyn Error>> {
    let conn = establish_connection();

    let teams: Vec<Team>;

    if let Some(market) = market {
        load_league(&conn, &market)?;
        teams = load_teams_by_league(&conn, &market.to_uppercase())?
    } else {
        teams = load_teams(&conn)?
    }
    Ok(teams)
}

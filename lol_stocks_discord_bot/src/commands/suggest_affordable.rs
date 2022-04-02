use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
    Args,
};

use std::error::Error;
use std::result::Result;

use rand::thread_rng;
use rand::seq::SliceRandom;

use lol_stocks_core::models::team::Team;

use lol_stocks_core::database::{
    connection::establish_connection,
    teams::load_teams,
    teams::load_teams_by_league,
    leagues::load_league,
    users::load_user,
};
use lol_stocks_core::errors::NoAffordableStock;
use crate::helpers::messages;

#[command]
pub async fn suggest_affordable(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let user_name = msg.author.name.clone();

    let mut team: Option<Team> = None;
    let market: Option<String>;
    let mut error_message: Option<String> = None;

    match args.single::<String>() {
        Ok(market_name) => market = Some(market_name.to_lowercase()),
        Err(_) => market = None
    }

    match get_team(&market, user_name) {
        Ok(t) => { team = Some(t) },
        Err(e) => { error_message = Some(e.to_string())}
    }

    if error_message.is_some() {
        messages::send_error_message(ctx, msg, error_message.unwrap()).await?;
    }

    if team.is_some() {
        let t = team.unwrap();
        messages::send_message::<&str, String>(
            ctx,
            msg,
            "Your randomly chosen team:",
            Some(&format!("{} (Price: {})", t.name, t.elo)),
            None
        ).await?;
    }

    Ok(())
}

fn get_team(market: &Option<String>, user_name: String) -> Result<Team, Box<dyn Error>> {
    let conn = establish_connection();
    let mut rng = thread_rng();
    let teams: Vec<Team>;
    let mut affordable_teams: Vec<Team> = vec![];

    let user = load_user(&conn, &user_name)?;

    if let Some(market) = market {
        load_league(&conn, market)?;
        teams = load_teams_by_league(&conn, &market.to_uppercase())?;
    } else {
        teams = load_teams(&conn)?;
    }

    for team in teams {
        if team.elo < user.balance {
            affordable_teams.push(team)
        }
    }

    if affordable_teams.is_empty() {
        return Err(Box::new(NoAffordableStock::new()))
    }

    let team = affordable_teams.choose(&mut rng).unwrap().clone();

    Ok(team)
}

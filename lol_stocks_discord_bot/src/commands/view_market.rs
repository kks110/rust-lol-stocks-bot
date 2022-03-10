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
    let mut teams: Vec<Team> = vec![];
    let market: Option<String>;
    let mut error_occurred: Option<String> = None;

    match args.single::<String>() {
        Ok(market_name) => market = Some(market_name.to_lowercase()),
        Err(_) => market = None
    }

    match make_view_market(&market) {
        Ok(t) => { teams = t },
        Err(e) => { error_occurred = Some(e.to_string())}
    }

    if error_occurred.is_some() {
        msg.channel_id.say(
            &ctx.http,
            format!("An Error as occurred: {}", error_occurred.unwrap().to_string())
        ).await?;
        return Ok(())
    }

    let market_name = market.unwrap_or("".to_string());

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            let mut message: String = "".to_string();
            for team in teams {
                message.push_str(&format!("**{}**: {}\n", team.name, team.elo));
            }
            e.description(message);

            if market_name == "lec" {
                e.thumbnail("attachment://lec.png");
            } else if market_name == "lcs" {
                e.thumbnail("attachment://lcs.png");
            } else {
                e.thumbnail("attachment://both.png");
            }
            e
        });

        if market_name == "lec" {
            m.add_file("./images/lec.png");
        } else if market_name == "lcs" {
            m.add_file("./images/lcs.png");
        } else {
            m.add_file("./images/both.png");
        }
        m
    }).await?;
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

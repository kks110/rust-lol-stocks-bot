use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
    Args,
};

use lol_stocks_core::database::{
    connection::establish_connection,
    teams::load_teams,
    teams::load_teams_by_league,
    leagues::load_leagues,
};
use lol_stocks_core::models::league::League;

#[command]
pub async fn view_market(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let conn = establish_connection();

    let mut response: String = String::from("");
    let mut teams = load_teams(&conn);
    let leagues: Option<Vec<League>>;

    match load_leagues(&conn){
        Ok(l) => leagues = Some(l),
        Err(e) => {
            leagues = None;
            response.push_str(&e.to_string())
        }
    };

    if leagues.is_some() {
        match args.single::<String>() {
            Ok(league_name) => {
                for league in leagues.unwrap() {
                    if league.name == league_name.to_uppercase() {
                        match load_teams_by_league(&conn, &league_name.to_uppercase()) {
                            Ok(t) => teams = t,
                            Err(e) => response.push_str(&e)
                        };
                    }
                }
            }
            Err(_) => {}
        }
    }

    for team in teams {
        let team_line = format!("{}  -  {}\n", team.name, team.elo);
        response.push_str(&team_line);
    }
    println!("Market displayed");
    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}

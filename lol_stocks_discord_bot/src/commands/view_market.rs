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
    teams::load_teams_be_league,
    leagues::load_leagues,
};

#[command]
pub async fn view_market(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let conn = establish_connection();

    let mut teams = load_teams(&conn);
    let leagues = load_leagues(&conn);

    match args.single::<String>() {
        Ok(league_name) => {
            for league in leagues {
                if league.name == league_name.to_uppercase() {
                    teams = load_teams_be_league(&conn, &league_name.to_uppercase());
                }
            }
        }
        Err(_) => {}
    }

    let mut response: String = String::from("");

    for team in teams {
        let team_line = format!("{}  -  {}\n", team.name, team.elo);
        response.push_str(&team_line);
    }
    println!("Market displayed");
    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}

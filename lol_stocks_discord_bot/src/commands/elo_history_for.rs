use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
    Args,
};

use std::error::Error;
use std::result::Result;

use lol_stocks_core::database::{
    connection::establish_connection,
    teams::load_team,
    team_elo_histories::load_team_elo_history,
};

#[command]
pub async fn elo_history_for(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let response: String;

    match parse_args(args) {
        Ok(t) => {
            let team_name = t;
            match load_elo_history(&team_name) {
                Ok(message) => { response = message },
                Err(e) => { response = format!("An error has occurred: {}", e.to_string())}
            }

        },
        Err(e) => { response = format!("An error as occurred {}", e.to_string()); }
    }


    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}

fn parse_args(mut args: Args) -> Result<String, Box<dyn Error>> {
    Ok(args.single::<String>()?)
}

fn load_elo_history(team_name: &str) -> Result<String, Box<dyn Error>> {
    let conn = establish_connection();
    let team = load_team(&conn, team_name)?;
    let team_elo_history = load_team_elo_history(&conn, &team)?;

    let mut message = format!("Date: Now, Value: {}\n", team.elo);

    for entry in team_elo_history {
        let response_line = format!("Team: {}, Date: {}, Price: {}\n", team.name, entry.date, entry.elo);
        message.push_str(&response_line)
    }
    Ok(message)
}

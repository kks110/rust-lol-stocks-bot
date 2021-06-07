use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{CommandResult, macros::command};

use lol_stocks_core::database::{
    connection::establish_connection,
    teams::load_teams,
};


#[command]
pub async fn view_market(ctx: &Context, msg: &Message) -> CommandResult {
    let conn = establish_connection();

    let teams = load_teams(&conn);

    let mut response: String = String::from("");

    for team in teams {
        let team_line = format!("{}  -  {}\n", team.name, team.elo);
        response.push_str(&team_line);
    }
    println!("Market displayed");
    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}

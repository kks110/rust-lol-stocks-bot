use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
    Args,
};

use lol_stocks_core::database::{
    connection::establish_connection,
    teams::load_team,
    team_elo_histories::load_team_elo_history,
};

#[command]
pub async fn elo_history_for(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let team_name = args.single::<String>()?;

    let conn = establish_connection();
    let team = load_team(&conn, &team_name);
    let team_elo_history = load_team_elo_history(&conn, &team, Option::from(true));

    let mut response = format!("Date: Now, Value: {}\n", team.elo);

    for entry in team_elo_history {
        let response_line = format!("Team: {}, Date: {}, Price: {}\n", team.name, entry.date, entry.elo);
        response.push_str(&response_line)
    }

    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}

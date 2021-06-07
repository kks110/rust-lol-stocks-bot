use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{CommandResult, macros::command, Args};

use lol_stocks_core::database::{
    connection::establish_connection,
    teams::{update_team, load_team}
};

use lol_stocks_core::elo::calculate_elo;

#[command]
pub async fn record_match(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let winner = args.single::<String>()?;
    let loser = args.single::<String>()?;

    let conn = establish_connection();
    let winning_team = load_team(&conn, &winner);
    let losing_team = load_team(&conn, &loser);

    let (winning_elo, losing_elo) = calculate_elo(winning_team.elo.clone(), losing_team.elo.clone());

    let winning_team = update_team(&conn, &winning_team.name, winning_elo);
    let losing_team = update_team(&conn, &losing_team.name, losing_elo);

    let response = format!("Recorded the match {} are now on {} ELO and {} are now on {} ELO", winning_team.name, winning_team.elo, losing_team.name, losing_team.elo);

    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}

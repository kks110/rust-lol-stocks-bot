use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
    Args,
};

use std::error::Error;
use std::result::Result;
use chrono::{NaiveDate, Utc};

use lol_stocks_core::database::{
    connection::establish_connection,
    teams::load_team,
    team_elo_histories::load_team_elo_history,
};
use crate::helpers::messages;
use crate::helpers::plus_sign::plus_sign;

struct HistoryData {
    pub date: NaiveDate,
    pub value: i32,
    pub difference: i32
}

#[command]
pub async fn team_history(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut entries: Option<Vec<HistoryData>> = None;
    let mut error_message: Option<String> = None;
    let mut team_name: String = "".to_string();

    match parse_args(args) {
        Ok(name) => { team_name = name },
        Err(e) => { error_message = Some(e.to_string()) }
    };

    match load_elo_history(&team_name) {
        Ok(message) => { entries = Some(message) },
        Err(e) => { error_message = Some(e.to_string()) }
    }

    if error_message.is_some() {
        messages::send_error_message(ctx, msg, error_message.unwrap()).await?;
    }

    if entries.is_some() {
        let mut fields: Vec<(String, String, bool)> = vec![];
        for entry in entries.unwrap() {
            fields.push(
                (
                    format!("{}", entry.date),
                    format!("{} ({}{})", entry.value, plus_sign(entry.difference), entry.difference),
                    false
                )
            )
        }
        messages::send_message::<&str, String>(
            ctx,
            msg,
            "Weekly Report:",
            None,
            Some(fields)
        ).await?;
    }

    Ok(())
}

fn parse_args(mut args: Args) -> Result<String, Box<dyn Error>> {
    Ok(args.single::<String>()?)
}

fn load_elo_history(team_name: &str) -> Result<Vec<HistoryData>, Box<dyn Error>> {
    let conn = establish_connection();
    let team = load_team(&conn, team_name)?;
    let team_elo_history = load_team_elo_history(&conn, &team)?;

    let mut history_data: Vec<HistoryData> = vec![
        HistoryData{
            date: Utc::now().date().naive_utc(),
            value: team.elo,
            difference: team.elo - team_elo_history.first().unwrap().elo
        }
    ];

    let mut counter = 1;

    for entry in &team_elo_history {
        let previous_value = match team_elo_history.get(counter) {
            Some(history) => history.elo,
            None => entry.elo
        };

        history_data.push(HistoryData{
            date: entry.date,
            value: entry.elo,
            difference: entry.elo - previous_value
        });
        counter += 1;
    }

    Ok(history_data)
}

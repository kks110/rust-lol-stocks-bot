use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

use std::error::Error;
use std::result::Result;

use crate::helpers::plus_sign::plus_sign;
use crate::helpers::messages;

use lol_stocks_core::{
    database::{
        connection::establish_connection,
        teams::load_teams,
        team_elo_histories::load_team_elo_history,
    }
};

struct WeeklyReportLine {
    pub team_name: String,
    pub current_elo: i32,
    pub difference: i32
}

#[command]
pub async fn weekly_report(ctx: &Context, msg: &Message) -> CommandResult {
    let mut weekly_report_lines: Option<Vec<WeeklyReportLine>> = None;
    let mut error_message: Option<String> = None;

    match make_weekly_report() {
        Ok(lines) => { weekly_report_lines = Some(lines) },
        Err(e) => { error_message = Some(e.to_string()) }
    }

    if error_message.is_some() {
        messages::send_error_message(ctx, msg, error_message.unwrap()).await?;
    }

    if weekly_report_lines.is_some() {
        let mut fields: Vec<(String, String, bool)> = vec![];
        for line in weekly_report_lines.unwrap() {
            fields.push(
                (
                    line.team_name,
                    format!("{} ({}{})", line.current_elo, plus_sign(line.difference), line.difference),
                    true
                )
            )
        }

        messages::send_message::<&str, String>(
            ctx,
            msg,
            "Weekly Report",
            None,
            Some(fields)
        ).await?;
    }

    Ok(())
}

fn make_weekly_report() -> Result<Vec<WeeklyReportLine>, Box<dyn Error>> {
    let conn = establish_connection();

    let teams = load_teams(&conn)?;

    let mut weekly_report_lines: Vec<WeeklyReportLine> = Vec::new();

    for team in teams {
        let current_price = team.elo;
        let previous_price = load_team_elo_history(&conn, &team)?.first().unwrap().elo;
        weekly_report_lines.push(WeeklyReportLine {
            team_name: team.name,
            current_elo: team.elo,
            difference: current_price - previous_price
        })
    }
    Ok(weekly_report_lines)
}
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::utils::Colour;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

use std::error::Error;
use std::result::Result;

use crate::helpers::plus_sign::plus_sign;

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
    let weekly_lines: Option<Vec<WeeklyReportLine>>;
    let response: Option<String>;

    match make_weekly_report() {
        Ok(weekly_report_lines) => {
            weekly_lines = Some(weekly_report_lines);
            response = None;
        },
        Err(e) => {
            response = Some(format!("An error a occurred: {}", e.to_string()));
            weekly_lines = None
        }
    }

    msg.channel_id.send_message(&ctx.http, |m| {
        if response.is_some() {
            m.content(response.unwrap());
        }
        if weekly_lines.is_some() {
            m.embed(|e| {
                e.title("Weekly Report:");
                e.colour(Colour::from_rgb(94, 166, 96));
                for line in weekly_lines.unwrap() {
                    e.field(line.team_name, format!("{} ({}{})", line.current_elo, plus_sign(line.difference), line.difference), true);
                }
                e
            });
        }
        m
    }).await?;
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
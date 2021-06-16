use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::utils::Colour;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

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
    let conn = establish_connection();

    let teams = load_teams(&conn);

    let mut weekly_report_lines: Vec<WeeklyReportLine> = Vec::new();

    for team in teams {
        let current_price = team.elo;
        let previous_price = load_team_elo_history(&conn, &team).first().unwrap().elo;
        weekly_report_lines.push(WeeklyReportLine {
            team_name: team.name,
            current_elo: team.elo,
            difference: current_price - previous_price
        })
    }

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Weekly Report:");
            e.colour(Colour::from_rgb(94, 166, 96));
            for line in weekly_report_lines {
                e.field(line.team_name, format!("{} ({})", line.current_elo, line.difference), true);
            }
            e
        })
    }).await?;
    Ok(())
}
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
    Args,
};
use std::env;
use std::error::Error;
use std::result::Result;

use lol_stocks_core::{
    database::{
        connection::establish_connection,
        teams::load_team,
        team_elo_histories::load_team_elo_history,
    }
};
use chrono::{Datelike, NaiveDate, Utc};
use graph_builder::models::{
    graph_data::GraphData,
    graph_data_point::GraphDataPoint
};
use crate::helpers::{
    messages,
    parse_args
};

#[command]
pub async fn team_history_graph(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut file_location: Option<String> = None;
    let mut error_message: Option<String> = None;

    match parse_args::parse_string(args) {
        Ok(team) => {
            let team_name = team;

            match make_team_graph(&team_name) {
                Ok(location) => { file_location = Some(location); },
                Err(e) => { error_message = Some(e.to_string()) }
            }
        },
        Err(e) => { error_message = Some(e.to_string()) }
    }

    if error_message.is_some() {
        messages::send_error_message(ctx, msg, error_message.unwrap()).await?;
    }

    if file_location.is_some() {
        messages::send_image_message(
            ctx,
            msg,
            file_location.unwrap()
        ).await?;
    }
    Ok(())
}

fn make_team_graph(team_name: &str) ->Result<String, Box<dyn Error>> {
    let conn = establish_connection();

    let team = load_team(&conn, team_name)?;
    let mut elo_history = load_team_elo_history(&conn, &team)?;
    elo_history.reverse();

    let mut graph_points: Vec<GraphDataPoint> = Vec::new();
    for entry in &elo_history {
        graph_points.push(GraphDataPoint::new(entry.date, entry.elo));
    }
    let todays_date = Utc::now();
    let todays_date = NaiveDate::from_ymd(todays_date.year(), todays_date.month(), todays_date.day());
    graph_points.push(GraphDataPoint{ x: todays_date, y: team.elo });

    let mut y_lowest_value: i32 = team.elo - 5;
    let mut y_highest_value: i32 = team.elo + 5;

    for point in &graph_points {
        if point.y - 5 < y_lowest_value {
            y_lowest_value = point.y - 5;
        }
        if point.y + 5 > y_highest_value {
            y_highest_value = point.y + 5;
        }
    }

    let mut earliest_date = graph_points.first().unwrap().x;
    let latest_date =  NaiveDate::from_ymd(Utc::now().year(), Utc::now().month(),Utc::now().day());

    for point in &graph_points {
        if point.x < earliest_date {
            earliest_date = point.x
        }
    }

    let mut file_location = env::var("GRAPH_LOCATION").expect("GRAPH_LOCATION must be set");
    let file_name = format!("/{}s_elo.png", team.name);
    file_location.push_str(&file_name);

    let data = GraphData::new(
        &file_location,
        &format!("{}s Elo", team.name),
        earliest_date,
        latest_date,
    "Week",
    y_lowest_value,
    y_highest_value,
    "Portfolio Price",
    graph_points
    );

    graph_builder::build(data)?;
    Ok(file_location)
}

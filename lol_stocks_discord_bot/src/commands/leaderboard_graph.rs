use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};
use std::env;
use std::error::Error;
use std::result::Result;
use crate::helpers::user_graph_data::graph_data_for_user;

use lol_stocks_core::{
    database::{
        connection::establish_connection,
        users::load_users,
    }
};

use graph_builder::models::{
    graph_data_multi_series::GraphDataMultiSeries,
    graph_data_point::GraphDataPoint,
    graph_data_series::GraphDataSeries
};

#[command]
pub async fn leaderboard_graph(ctx: &Context, msg: &Message) -> CommandResult {
    let response: Option<String>;
    let file_location: Option<String>;

    match make_leaderboard_graph() {
        Ok(location) => {
            file_location = Some(location);
            response = None;
        },
        Err(e) => {
            response = Some(format!("An error has occurred: {}", e.to_string()));
            file_location = None;
        }
    }

    msg.channel_id.send_message(&ctx.http, |m| {
        if let Some(response) = response {
            m.content(response);
        }
        if file_location.is_some() {
            m.add_file(&file_location.as_ref().unwrap()[..]);
        }
        m
    }).await?;

    Ok(())
}

fn make_leaderboard_graph() -> Result<String, Box<dyn Error>> {
    let conn = establish_connection();

    let users = load_users(&conn)?;

    let mut data_series: Vec<GraphDataSeries> = vec![];

    let mut y_lowest_value: i32 = 5000;
    let mut y_highest_value: i32 = 5000;
    let mut y_length = 0;

    for user in users {
        let graph_points: Vec<GraphDataPoint> = graph_data_for_user(&user)?;
        for point in &graph_points {
            if point.y - 50 < y_lowest_value {
                y_lowest_value = point.y - 50;
            }
            if point.y + 50 > y_highest_value {
                y_highest_value = point.y + 50;
            }
        }
        if graph_points.len() > y_length {
            y_length = graph_points.len();
        }
        data_series.push(GraphDataSeries{ name: user.name, series: graph_points });
    }

    let mut file_location = env::var("GRAPH_LOCATION").expect("GRAPH_LOCATION must be set");
    file_location.push_str("/leaderboard_graph.png");

    let data = GraphDataMultiSeries {
        file_name: file_location.clone(),
        graph_name: String::from("Leaderboard"),
        x_lower: 1,
        x_upper: y_length as i32,
        x_description: "Week".to_string(),
        y_lower: y_lowest_value,
        y_upper: y_highest_value,
        y_description: String::from("Portfolio Price"),
        data: data_series
    };

    graph_builder::build_multi_series(data);
    Ok(file_location)
}

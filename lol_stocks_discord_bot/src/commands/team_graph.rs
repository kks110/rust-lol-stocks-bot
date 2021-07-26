use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
    Args,
};
use std::env;

use lol_stocks_core::{
    database::{
        connection::establish_connection,
        teams::load_team,
        team_elo_histories::load_team_elo_history,
    }
};

use graph_builder::models::{
    graph_data::GraphData,
    graph_data_point::GraphDataPoint
};

#[command]
pub async fn team_graph(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let conn = establish_connection();
    let team_name = args.single::<String>()?;

    let team = load_team(&conn, &team_name);
    let mut elo_history = load_team_elo_history(&conn, &team, None);
    elo_history.reverse();

    let mut graph_points: Vec<GraphDataPoint> = Vec::new();
    let mut week_number = 1;
    for entry in &elo_history {
        graph_points.push(GraphDataPoint{ x: week_number, y: entry.elo });
        week_number += 1;
    }
    graph_points.push(GraphDataPoint{ x: week_number, y: team.elo });

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

    let mut file_location = env::var("GRAPH_LOCATION").expect("GRAPH_LOCATION must be set");
    let file_name = format!("/{}s_elo.png", team.name);
    file_location.push_str(&file_name);

    let data = GraphData {
        file_name: file_location.clone(),
        graph_name: format!("{}s Elo", team.name),
        x_lower: 1,
        x_upper: (elo_history.len() + 1) as i32,
        x_description: "Week".to_string(),
        y_lower: y_lowest_value,
        y_upper: y_highest_value,
        y_description: "Portfolio Price".to_string(),
        data: graph_points
    };

    graph_builder::build(data);

    msg.channel_id.send_message(&ctx.http, |m| {
        m.add_file(&file_location[..])
    }).await?;

    Ok(())
}

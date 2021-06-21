use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};
use std::env;

use lol_stocks_core::{
    portfolio_calculations::calculate_portfolio_value,
    database::{
        connection::establish_connection,
        users::load_user,
        user_portfolio_histories::load_user_portfolio_history,
        portfolios::load_users_portfolio,
    }
};

use graph_builder::models::{
    graph_data::GraphData,
    graph_data_point::GraphDataPoint
};

#[command]
pub async fn portfolio_graph(ctx: &Context, msg: &Message) -> CommandResult {
    let conn = establish_connection();
    let user_name = msg.author.name.clone();

    let user = load_user(&conn, &user_name);
    let mut portfolio_history = load_user_portfolio_history(&conn, &user);
    portfolio_history.reverse();

    let portfolio = load_users_portfolio(&conn, &user);
    let current_value = calculate_portfolio_value(&conn, &user, &portfolio);

    let mut graph_points: Vec<GraphDataPoint> = Vec::new();
    let mut week_number = 1;
    for entry in &portfolio_history {
        graph_points.push(GraphDataPoint{ x: week_number, y: entry.value });
        week_number += 1;
    }
    graph_points.push(GraphDataPoint{ x: week_number, y: current_value });

    let mut y_lowest_value: i32 = current_value - 50;
    let mut y_highest_value: i32 = current_value + 50;

    for point in &graph_points {
        if point.y - 50 < y_lowest_value {
            y_lowest_value = point.y - 50;
        }
        if point.y + 50 > y_highest_value {
            y_highest_value = point.y + 50;
        }
    }

    let mut file_location = env::var("GRAPH_LOCATION").expect("GRAPH_LOCATION must be set");
    let file_name = format!("/{}s_portfolio.png", user.name);
    file_location.push_str(&file_name);

    let data = GraphData {
        file_name: file_location.clone(),
        graph_name: format!("{}s Portfolio", user.name),
        x_lower: 1,
        x_upper: (portfolio_history.len() + 1) as i32,
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

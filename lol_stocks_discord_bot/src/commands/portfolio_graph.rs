use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};
use std::env;
use std::error::Error;
use std::result::Result;
use chrono::{Datelike, NaiveDate, Utc};

use crate::helpers::user_graph_data::graph_data_for_user;

use lol_stocks_core::{
    portfolio_calculations::calculate_portfolio_value,
    database::{
        connection::establish_connection,
        users::load_user,
        portfolios::load_users_portfolio,
    }
};

use graph_builder::models::{
    graph_data::GraphData,
    graph_data_point::GraphDataPoint
};

#[command]
pub async fn portfolio_graph(ctx: &Context, msg: &Message) -> CommandResult {
    let user_name = msg.author.name.clone();

    let response: String;
    let mut file_location = "".to_string();

    match make_portfolio_graph(&user_name) {
        Ok(location) => {
            response = "".to_string();
            file_location.push_str(&location)
        },
        Err(e) => { response = format!("An error has occurred: {}", e.to_string()) }
    }

    msg.channel_id.send_message(&ctx.http, |m| {
        m.content(response);
        m.add_file(&file_location[..])
    }).await?;

    Ok(())
}

fn make_portfolio_graph(user_name: &str) -> Result<String, Box<dyn Error>> {
    let conn = establish_connection();

    let user = load_user(&conn, user_name)?;
    let portfolio = load_users_portfolio(&conn, &user)?;
    let graph_points: Vec<GraphDataPoint> = graph_data_for_user(&user)?;

    let current_value = calculate_portfolio_value(&conn, &user, &portfolio)?;
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

    let mut earliest_date = graph_points.first().unwrap().x;
    let latest_date =  NaiveDate::from_ymd(Utc::now().year(), Utc::now().month(),Utc::now().day());

    for point in &graph_points {
        if point.x < earliest_date {
            earliest_date = point.x
        }
    }

    let mut file_location = env::var("GRAPH_LOCATION").expect("GRAPH_LOCATION must be set");
    let file_name = format!("/{}s_portfolio.png", user.name);
    file_location.push_str(&file_name);

    let data = GraphData::new(
        &file_location,
        &format!("{}s Portfolio", user.name),
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

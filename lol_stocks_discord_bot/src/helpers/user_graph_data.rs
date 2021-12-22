use lol_stocks_core::{
    portfolio_calculations::calculate_portfolio_value,
    database::{
        connection::establish_connection,
        user_portfolio_histories::load_user_portfolio_history,
        portfolios::load_users_portfolio,
    },
    models::user::User,
};

use graph_builder::models::{
    graph_data_point::GraphDataPoint
};
use chrono::{Datelike, NaiveDate, Utc};
use std::error::Error;


pub fn graph_data_for_user(user: &User) -> Result<Vec<GraphDataPoint>, Box<dyn Error>> {
    let conn = establish_connection();

    let mut portfolio_history = load_user_portfolio_history(&conn, user, None)?;
    portfolio_history.reverse();

    let portfolio = load_users_portfolio(&conn, user)?;
    let current_value = calculate_portfolio_value(&conn, user, &portfolio)?;

    let mut graph_points: Vec<GraphDataPoint> = Vec::new();
    for entry in &portfolio_history {
        graph_points.push(GraphDataPoint::new(entry.date, entry.value));
    }
    let todays_date = Utc::now();
    let todays_date = NaiveDate::from_ymd(todays_date.year(), todays_date.month(), todays_date.day());
    graph_points.push(GraphDataPoint::new(todays_date, current_value));
    Ok(graph_points)
}

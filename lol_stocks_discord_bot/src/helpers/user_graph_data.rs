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


pub fn graph_data_for_user(user: &User) -> Vec<GraphDataPoint> {
    let conn = establish_connection();

    let mut portfolio_history = load_user_portfolio_history(&conn, &user, None);
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
    graph_points
}

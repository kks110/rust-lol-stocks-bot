use crate::database::{
    connection::establish_connection,
    teams::load_teams,
    team_elo_histories::create_team_elo_history,
    users::load_users,
    user_portfolio_histories::create_user_portfolio_history,
    portfolios::load_users_portfolio,
};

use crate::portfolio_calculations::calculate_portfolio_value;

pub fn take_history_snapshot() {
    update_team_history();
    update_user_history()
}

fn update_team_history() {
    let conn = establish_connection();

    let teams = load_teams(&conn);
    for team in teams {
        create_team_elo_history(&conn, &team.elo, &team.id);
    }
}

fn update_user_history() {
    let conn = establish_connection();

    let users = load_users(&conn);
    for user in users {
        let portfolio = load_users_portfolio(&conn, &user);
        let portfolio_value = calculate_portfolio_value(&conn, &user, &portfolio);

        create_user_portfolio_history(&conn, &portfolio_value, &user.id);
    }
}

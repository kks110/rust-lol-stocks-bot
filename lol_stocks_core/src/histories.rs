use crate::database::{
    connection::establish_connection,
    teams::load_teams,
    team_elo_histories::create_team_elo_history,
    users::load_users,
    user_portfolio_histories::create_user_portfolio_history,
    portfolios::load_users_portfolio,
};

use crate::portfolio_calculations::calculate_portfolio_value;

pub fn take_history_snapshot() -> Result<String, String> {
    match update_team_history() {
        Ok(_) => {},
        Err(e) => return Err(e)
    };
    match update_user_history() {
        Ok(_) => {},
        Err(e) => return Err(e)
    };
    Ok("".to_string())
}

fn update_team_history() -> Result<String, String> {
    let conn = establish_connection();

    let teams = match load_teams(&conn) {
        Ok(t) => t,
        Err(e) => return Err(e)
    };

    for team in teams {
        create_team_elo_history(&conn, &team.elo, &team.id);
    }
    Ok("".to_string())
}

fn update_user_history() -> Result<String, String> {
    let conn = establish_connection();

    let users = load_users(&conn);
    for user in users {
        let portfolio = load_users_portfolio(&conn, &user);
        let portfolio_value = match calculate_portfolio_value(&conn, &user, &portfolio) {
            Ok(value) => value,
            Err(e) => return Err(e)
        };

        create_user_portfolio_history(&conn, &portfolio_value, &user.id);
    }
    Ok("".to_string())
}

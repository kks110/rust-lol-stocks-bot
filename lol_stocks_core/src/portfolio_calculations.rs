use crate::models::{
    user::User,
    portfolio::Portfolio
};

use crate::database::{
    teams::load_team_by_id,
};

use diesel::PgConnection;

pub fn calculate_portfolio_value(conn: &PgConnection, user: &User, portfolio: &Vec<Portfolio>) -> i32 {
    let mut value = 0;
    value = value + user.balance;

    for holding in portfolio {
        let team = load_team_by_id(&conn, &holding.team_id);
        let holding_value = team.elo * holding.amount;
        value = value + holding_value;
    }
    value
}

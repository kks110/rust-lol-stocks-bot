use crate::models::{
    user::User,
    portfolio::Portfolio
};

use crate::database::{
    teams::load_team_by_id,
};

use std::error::Error;
use diesel::PgConnection;

pub fn calculate_portfolio_value(conn: &PgConnection, user: &User, portfolio: &[Portfolio]) -> Result<i32, Box<dyn Error>> {
    let mut value = user.balance;

    for holding in portfolio {
        let team = load_team_by_id(conn, &holding.team_id)?;
        let holding_value = team.elo * holding.amount;
        value += holding_value;
    }
    Ok(value)
}

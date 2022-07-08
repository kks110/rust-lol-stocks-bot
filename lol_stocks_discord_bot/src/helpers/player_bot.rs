use std::error::Error;
use rand::thread_rng;
use rand::seq::SliceRandom;
use lol_stocks_core::database::connection::establish_connection;
use lol_stocks_core::database::portfolios::{load_users_portfolio, user_portfolio_purchase, user_portfolio_sell};
use lol_stocks_core::database::teams::{load_team_by_id, load_teams};
use lol_stocks_core::database::users::{load_user, update_user};
use lol_stocks_core::errors::NoAffordableStock;
use lol_stocks_core::models::team::Team;
use lol_stocks_core::models::user::User;
use lol_stocks_core::portfolio_calculations::calculate_portfolio_value;

pub fn get_bot_portfolio() -> Result<String, Box<dyn Error>> {
    let conn = establish_connection();

    let user = load_user(&conn, "StockBot")?;
    let users_portfolio = load_users_portfolio(&conn, &user)?;
    let p = users_portfolio.first().unwrap();
    let team = load_team_by_id(&conn, &p.team_id)?;

    Ok(format!("🤖 💹 StockBot has bought {} shares in {}!", p.amount, team.name))
}


pub fn sell_all() -> Result<(), Box<dyn Error>> {
    let conn = establish_connection();

    let user = load_user(&conn, "StockBot")?;
    let users_portfolio = load_users_portfolio(&conn, &user)?;

    let new_balance = calculate_portfolio_value(&conn, &user, &users_portfolio)?;
    update_user(&conn, &user.name, new_balance)?;
    for portfolio in users_portfolio {
        let team = load_team_by_id(&conn, &portfolio.team_id)?;
        user_portfolio_sell(&conn, &user, &team, portfolio.amount)?;
    }
    Ok(())
}


pub fn buy_all() -> Result<(), Box<dyn Error>> {
    let conn = establish_connection();

    let user = load_user(&conn, "StockBot")?;
    let team = get_team(&user)?;

    let amount = user.balance / team.elo;

    let cost = team.elo * amount;
    update_user(&conn, &user.name, user.balance - cost)?;
    user_portfolio_purchase(&conn, &user, &team, amount)?;
    Ok(())
}


fn get_team(user: &User) -> Result<Team, Box<dyn Error>> {
    let conn = establish_connection();
    let mut rng = thread_rng();
    let teams: Vec<Team>;
    let mut affordable_teams: Vec<Team> = vec![];

    teams = load_teams(&conn)?;

    for team in teams {
        if team.elo < user.balance {
            affordable_teams.push(team)
        }
    }

    if affordable_teams.is_empty() {
        return Err(Box::new(NoAffordableStock::new()))
    }

    let team = affordable_teams.choose(&mut rng).unwrap().clone();

    Ok(team)
}

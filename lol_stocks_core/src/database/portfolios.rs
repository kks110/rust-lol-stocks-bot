use diesel::prelude::*;
use crate::models::portfolio::{Portfolio, NewPortfolio};
use crate::models::user::User;
use crate::models::team::Team;
use std::error::Error;

pub fn load_users_portfolio(conn: &PgConnection, user: &User) -> Result<Vec<Portfolio>, Box<dyn Error>> {
    Ok(Portfolio::belonging_to(user).load::<Portfolio>(conn)?)
}

pub fn load_all_portfolios(conn: &PgConnection) -> Result<Vec<Portfolio>, Box<dyn Error>> {
    use crate::schema::portfolios::dsl::*;

    Ok(portfolios.load::<Portfolio>(conn)?)
}

pub fn create_portfolio(conn: &PgConnection, team_id: i32, user_id: i32, amount: i32) -> Result<Portfolio, Box<dyn Error>> {
    use crate::schema::portfolios;

    let new_portfolio = NewPortfolio::new(team_id, user_id, amount);

    Ok(diesel::insert_into(portfolios::table)
        .values(&new_portfolio)
        .get_result(conn)?
    )
}

pub fn user_portfolio_purchase(conn: &PgConnection, purchasing_user: &User, team_purchased: &Team, amount_purchased: i32) -> Result<Portfolio, Box<dyn Error>> {
    use crate::schema::portfolios::dsl::*;

    let users_portfolio: Vec<Portfolio> = Portfolio::belonging_to(purchasing_user).load::<Portfolio>(conn)?;
    for portfolio in users_portfolio {
        if portfolio.team_id == team_purchased.id {
             return Ok(diesel::update(portfolios.filter(id.eq(portfolio.id)))
                .set(amount.eq(portfolio.amount + amount_purchased))
                .get_result::<Portfolio>(conn)?
             )
        }
    }
    create_portfolio(conn, team_purchased.id, purchasing_user.id, amount_purchased)
}

pub fn user_portfolio_sell(conn: &PgConnection, selling_user: &User, team_being_sold: &Team, amount_sold: i32) -> Result<(), Box<dyn Error>> {
    use crate::schema::portfolios::dsl::*;

    let users_portfolio: Vec<Portfolio> = load_users_portfolio(conn, selling_user)?;

    for portfolio in users_portfolio {
        if portfolio.team_id == team_being_sold.id {
            if portfolio.amount - amount_sold == 0 {
                delete_portfolio(conn, portfolio.id)?;
            } else {
                diesel::update(portfolios.filter(id.eq(portfolio.id)))
                    .set(amount.eq(portfolio.amount - amount_sold))
                    .get_result::<Portfolio>(conn)?;
            }
        }
    }
    Ok(())
}

fn delete_portfolio(conn: &PgConnection, portfolio_id: i32) -> Result<(), Box<dyn Error>> {
    use crate::schema::portfolios::dsl::*;
    diesel::delete(portfolios.filter(id.eq(portfolio_id))).execute(conn)?;
    Ok(())
}

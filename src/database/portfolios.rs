use crate::database::connection;
use crate::diesel::prelude::*;
use crate::models::portfolio::{Portfolio, NewPortfolio};
use crate::models::user::User;
use crate::models::team::Team;

pub fn load_portfolios(conn: &PgConnection) -> Vec<Portfolio>  {
    use crate::schema::portfolios::dsl::*;

    portfolios.load::<Portfolio>(conn).expect("Error loading portfolios")
}

pub fn load_users_portfolio(conn: &PgConnection, user: &User) -> Vec<Portfolio> {
    use crate::schema::portfolios::dsl::*;
    use crate::schema::portfolios;
    use crate::schema::users;

    Portfolio::belonging_to(user).load::<Portfolio>(conn).expect("Error loading portfolios")
}

pub fn create_portfolio<'a>(conn: &PgConnection, team_id: &'a i32, user_id: &'a i32, amount: &'a i32) -> Portfolio {
    use crate::schema::portfolios;

    let new_portfolio = NewPortfolio {
        team_id,
        user_id,
        amount,
    };

    diesel::insert_into(portfolios::table)
        .values(&new_portfolio)
        .get_result(conn)
        .expect("Error saving new portfolio")
}

pub fn user_portfolio_purchase<'a>(conn: &PgConnection, purchasing_user: &User, team_purchased: &Team, amount_purchased: i32) -> Portfolio {
    use crate::schema::portfolios::dsl::*;

    let users_portfolio: Vec<Portfolio> = Portfolio::belonging_to(purchasing_user).load::<Portfolio>(conn).expect("Error loading portfolios");
    for portfolio in users_portfolio {
        if portfolio.team_id == team_purchased.id {
            return diesel::update(portfolios.filter(id.eq(portfolio.id)))
                .set(amount.eq(portfolio.amount + amount_purchased))
                .get_result::<Portfolio>(conn)
                .expect(&format!("Unable to find portfolio for user: {}", purchasing_user.name))
        }
    }
    return create_portfolio(conn, &team_purchased.id, &purchasing_user.id, &amount_purchased);
}

pub fn user_portfolio_sell<'a>(conn: &PgConnection, selling_user: &User, team_being_sold: &Team, amount_sold: i32) -> Portfolio {
    use crate::schema::portfolios::dsl::*;

    let users_portfolio: Vec<Portfolio> = load_users_portfolio(conn, selling_user);
    let mut port: Portfolio = Portfolio {
        id: 0,
        team_id: 0,
        user_id: 0,
        amount: 0
    };

    for portfolio in users_portfolio {
        if portfolio.team_id == team_being_sold.id {
            if portfolio.amount - amount_sold == 0 {
                delete_portfolio(conn, portfolio.id);
            } else {
                return diesel::update(portfolios.filter(id.eq(portfolio.id)))
                    .set(amount.eq(portfolio.amount - amount_sold))
                    .get_result::<Portfolio>(conn)
                    .expect(&format!("Unable to find portfolio for user: {}", selling_user.name));
            }
        }
    }

    return port;
}

fn delete_portfolio(conn: &PgConnection, portfolio_id: i32) {
    use crate::schema::portfolios::dsl::*;
    diesel::delete(portfolios.filter(id.eq(portfolio_id))).execute(conn).expect("Could not delete");
}

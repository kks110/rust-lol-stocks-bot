use diesel::prelude::*;
use crate::models::user_portfolio_history::{UserPortfolioHistory, NewUserPortfolioHistory};
use crate::models::user::User;
use std::error::Error;

pub fn load_user_portfolio_history(conn: &PgConnection, user: &User, limit: Option<i64>) -> Result<Vec<UserPortfolioHistory>, Box<dyn Error>> {
    use crate::schema::user_portfolio_histories::dsl::*;

    let return_limit = limit.unwrap_or(0);

    if return_limit > 0 {
        Ok(UserPortfolioHistory::belonging_to(user)
            .order(date.desc())
            .limit(return_limit)
            .load::<UserPortfolioHistory>(conn)?
        )
    } else {
        Ok(UserPortfolioHistory::belonging_to(user)
            .order(date.desc())
            .load::<UserPortfolioHistory>(conn)?
        )
    }
}

pub fn create_user_portfolio_history<'a>(conn: &PgConnection, value: &'a i32, user_id: &'a i32) -> Result<UserPortfolioHistory, Box<dyn Error>> {
    use crate::schema::user_portfolio_histories;

    let new_user_portfolio_history = NewUserPortfolioHistory {
        value,
        user_id
    };

    Ok(diesel::insert_into(user_portfolio_histories::table)
        .values(&new_user_portfolio_history)
        .get_result(conn)?
    )
}

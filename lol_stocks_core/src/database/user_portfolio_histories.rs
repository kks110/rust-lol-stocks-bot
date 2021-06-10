use diesel::prelude::*;
use crate::models::user_portfolio_history::{UserPortfolioHistory, NewUserPortfolioHistory};
use crate::models::user::User;

pub fn load_user_portfolio_history(conn: &PgConnection, user: &User) -> Vec<UserPortfolioHistory> {
    use crate::schema::user_portfolio_histories::dsl::*;

    UserPortfolioHistory::belonging_to(user)
        .order(week.desc())
        .limit(5)
        .load::<UserPortfolioHistory>(conn)
        .expect("Error loading portfolio history")
}

pub fn create_user_portfolio_history<'a>(conn: &PgConnection, week: &'a i32, value: &'a i32, user_id: &'a i32) -> UserPortfolioHistory {
    use crate::schema::user_portfolio_histories;

    let new_user_portfolio_history = NewUserPortfolioHistory {
        week,
        value,
        user_id
    };

    diesel::insert_into(user_portfolio_histories::table)
        .values(&new_user_portfolio_history)
        .get_result(conn)
        .expect("Error saving portfolio history")
}

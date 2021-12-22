use crate::schema::user_portfolio_histories;
use crate::models::user::User;
use chrono::NaiveDate;

#[derive(Identifiable, Queryable, Associations)]
#[table_name="user_portfolio_histories"]
#[belongs_to(User)]
pub struct UserPortfolioHistory {
    pub id: i32,
    pub date: NaiveDate,
    pub value: i32,
    pub user_id: i32,
}

#[derive(Insertable)]
#[table_name="user_portfolio_histories"]
pub struct NewUserPortfolioHistory {
    pub value: i32,
    pub user_id: i32,
}

impl NewUserPortfolioHistory {
    pub fn new(value: i32, user_id: i32) -> NewUserPortfolioHistory {
        NewUserPortfolioHistory {
            value,
            user_id
        }
    }
}
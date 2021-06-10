use crate::schema::user_portfolio_histories;
use crate::models::user::User;

#[derive(Identifiable, Queryable, Associations)]
#[table_name="user_portfolio_histories"]
#[belongs_to(User)]
pub struct UserPortfolioHistory {
    pub id: i32,
    pub week: i32,
    pub value: i32,
    pub user_id: i32,
}

#[derive(Insertable)]
#[table_name="user_portfolio_histories"]
pub struct NewUserPortfolioHistory<'a> {
    pub week: &'a i32,
    pub value: &'a i32,
    pub user_id: &'a i32,
}

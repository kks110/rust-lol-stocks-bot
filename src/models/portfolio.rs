use crate::schema::portfolios;
use crate::models::user::User;

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(User, Team)]
pub struct Portfolio {
    pub id: i32,
    pub team_id: i32,
    pub user_id: i32,
    pub amount: i32
}

#[derive(Insertable)]
#[table_name="portfolios"]
pub struct NewPortfolio<'a> {
    pub team_id: &'a i32,
    pub user_id: &'a i32,
    pub amount: &'a i32,
}

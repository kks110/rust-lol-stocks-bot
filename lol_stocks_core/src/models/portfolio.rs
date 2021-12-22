use crate::schema::portfolios;
use crate::models::user::User;

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(User)]
pub struct Portfolio {
    pub id: i32,
    pub team_id: i32,
    pub user_id: i32,
    pub amount: i32
}

#[derive(Insertable)]
#[table_name="portfolios"]
pub struct NewPortfolio {
    pub team_id: i32,
    pub user_id: i32,
    pub amount: i32,
}

impl NewPortfolio {
    pub fn new(team_id: i32, user_id: i32, amount: i32) -> NewPortfolio {
        NewPortfolio {
            team_id,
            user_id,
            amount
        }
    }
}
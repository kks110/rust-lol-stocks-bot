use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersList {
    pub users: Vec<User>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub username: String,
    pub balance: i64,
    pub portfolio: Vec<Stock>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stock {
    pub team: String,
    pub amount: i64,
}

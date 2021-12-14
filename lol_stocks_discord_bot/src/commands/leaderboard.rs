use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

use std::error::Error;

use lol_stocks_core::{
    portfolio_calculations::calculate_portfolio_value,
    database::{
        connection::establish_connection,
        users::load_users,
        portfolios::load_users_portfolio,
    },
};

struct LeaderboardEntry {
    pub user_name: String,
    pub value: i32
}

#[command]
pub async fn leaderboard(ctx: &Context, msg: &Message) -> CommandResult {
    let response: String;

    match load_leaderboard() {
        Ok(message) => { response = message },
        Err(e) => { response = format!("An Error as occurred: {}", e.to_string()) }
    }

    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}

fn load_leaderboard() -> Result<String, Box<dyn Error>> {
    let conn = establish_connection();

    let users = load_users(&conn)?;
    let mut leaderboard_entries: Vec<LeaderboardEntry> = Vec::new();

    for user in users {
        let portfolio = load_users_portfolio(&conn, &user)?;
        let current_value = calculate_portfolio_value(&conn, &user, &portfolio)?;
        leaderboard_entries.push(LeaderboardEntry {
            user_name: user.name,
            value: current_value
        })
    }

    leaderboard_entries.sort_by(|a, b| b.value.cmp(&a.value));

    let mut message = String::from("Leaderboard:\n");
    for entry in leaderboard_entries {
        let entry_string = format!("{}: {}\n", entry.user_name, entry.value);
        message.push_str(&entry_string);
    }
    Ok(message)
}

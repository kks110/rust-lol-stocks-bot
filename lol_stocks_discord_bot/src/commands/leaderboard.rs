use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

use std::error::Error;
use std::result::Result;

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
    let mut entries: Vec<LeaderboardEntry> = vec![];
    let mut error_occurred: Option<String> = None;

    match load_leaderboard() {
        Ok(message) => { entries = message },
        Err(e) => { error_occurred = Some(e.to_string()) }
    }

    if error_occurred.is_some() {
        msg.channel_id.say(
            &ctx.http,
            format!("An Error as occurred: {}", error_occurred.unwrap().to_string())
        ).await?;
        return Ok(())
    }

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            let mut response = vec![];
            for (index, player) in entries.iter().enumerate() {
                response.push((format!("{}. {}: ", index + 1, player.user_name), format!("{}", player.value), false))
            };
            e
                .title("Leaderboard:".to_string())
                .fields(response)
        })
    }).await?;
    Ok(())
}

fn load_leaderboard() -> Result<Vec<LeaderboardEntry>, Box<dyn Error>> {
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

    Ok(leaderboard_entries)
}

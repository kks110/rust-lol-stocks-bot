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
use lol_stocks_core::database::user_portfolio_histories::load_user_portfolio_history;
use crate::helpers::messages;
use crate::helpers::plus_sign::plus_sign;

struct LeaderboardEntry {
    pub user_name: String,
    pub value: i32,
    pub difference: i32,
}

#[command]
pub async fn leaderboard(ctx: &Context, msg: &Message) -> CommandResult {
    let mut entries: Option<Vec<LeaderboardEntry>> = None;
    let mut error_message: Option<String> = None;

    match load_leaderboard() {
        Ok(message) => { entries = Some(message) },
        Err(e) => { error_message = Some(e.to_string()) }
    }

    if error_message.is_some() {
        messages::send_error_message(ctx, msg, error_message.unwrap()).await?;
    }

    if entries.is_some() {
        let mut fields: Vec<(String, String, bool)> = vec![];
        for (index, player) in entries.unwrap().iter().enumerate() {
            fields.push(
                (
                    format!("{}. {}: ", index + 1, player.user_name),
                    format!("{} ({}{})", player.value, plus_sign(player.difference), player.difference),
                    false
                )
            )
        }

        messages::send_message::<&str, String>(
            ctx,
            msg,
            "Leaderboard",
            None,
            Some(fields)
        ).await?;
    }

    Ok(())
}

fn load_leaderboard() -> Result<Vec<LeaderboardEntry>, Box<dyn Error>> {
    let conn = establish_connection();

    let users = load_users(&conn)?;
    let mut leaderboard_entries: Vec<LeaderboardEntry> = Vec::new();

    for user in users {
        let portfolio = load_users_portfolio(&conn, &user)?;
        let value = calculate_portfolio_value(&conn, &user, &portfolio)?;
        let previous_value = load_user_portfolio_history(&conn, &user, None)?.first().unwrap().value;
        leaderboard_entries.push(LeaderboardEntry {
            user_name: user.name,
            value,
            difference: value - previous_value
        })
    }

    leaderboard_entries.sort_by(|a, b| b.value.cmp(&a.value));

    Ok(leaderboard_entries)
}

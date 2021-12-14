use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

use chrono::{
    offset::Utc,
    NaiveDate
};

use std::error::Error;

use crate::helpers::plus_sign::plus_sign;

use lol_stocks_core::{
    portfolio_calculations::calculate_portfolio_value,
    database::{
        connection::establish_connection,
        users::load_user,
        user_portfolio_histories::load_user_portfolio_history,
        portfolios::load_users_portfolio,
    }
};

struct HistoryData {
    pub date: NaiveDate,
    pub value: i32,
    pub difference: i32
}

#[command]
pub async fn portfolio_performance(ctx: &Context, msg: &Message) -> CommandResult {
    let user_name = msg.author.name.clone();

    let response: String;

    match make_portfolio_performance(&user_name) {
        Ok(message) => { response = message},
        Err(e) => { response = format!("An error has occurred: {}", e.to_string())}
    }

    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}

fn make_portfolio_performance(user_name: &str) -> Result<String, Box<dyn Error>> {
    let conn = establish_connection();
    let user = load_user(&conn, &user_name)?;
    let user_portfolio_history = load_user_portfolio_history(&conn, &user, Option::from(5))?;

    let portfolio = load_users_portfolio(&conn, &user)?;
    let current_value = calculate_portfolio_value(&conn, &user, &portfolio)?;

    let mut history_data: Vec<HistoryData> = Vec::new();

    history_data.push(HistoryData{
        date: Utc::now().date().naive_utc(),
        value: current_value,
        difference: current_value - user_portfolio_history.first().unwrap().value
    });

    let mut counter = 1;

    for portfolio_history in &user_portfolio_history {
        let previous_value = match user_portfolio_history.get(counter) {
            Some(history) => history.value,
            None => portfolio_history.value
        };

        history_data.push(HistoryData{
            date: portfolio_history.date,
            value: portfolio_history.value,
            difference: portfolio_history.value - previous_value
        });
        counter += 1;
    }

    let mut  message = String::from("");

    for entry in history_data {
        let response_line = format!("Date: {}, Value: {} ({}{})\n", entry.date, entry.value, plus_sign(entry.difference), entry.difference);
        message.push_str(&response_line)
    }
    Ok(message)
}

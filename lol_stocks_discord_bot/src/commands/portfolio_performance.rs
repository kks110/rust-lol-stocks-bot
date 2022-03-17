use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
    Args,
};

use chrono::{
    offset::Utc,
    NaiveDate
};

use std::error::Error;
use std::result::Result;

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
use crate::helpers::send_error::send_error;

struct HistoryData {
    pub date: NaiveDate,
    pub value: i32,
    pub difference: i32
}

#[command]
pub async fn portfolio_performance(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let user_name = match args.single::<String>() {
        Ok(user) => user,
        Err(_) => msg.author.name.clone()
    };

    let mut entries: Vec<HistoryData> = vec![];
    let mut error_occurred: Option<String> = None;

    match make_portfolio_performance(&user_name) {
        Ok(message) => { entries = message },
        Err(e) => { error_occurred = Some(e.to_string()) }
    }

    if error_occurred.is_some() {
        send_error(ctx, msg, error_occurred.unwrap()).await?;
        return Ok(())
    }

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            let mut response = vec![];
            for entry in entries {
                response.push((format!("{}", entry.date), format!("{} ({}{})", entry.value, plus_sign(entry.difference), entry.difference), false ))
            };
            e
                .title(format!("{}'s portfolio performance", user_name))
                .fields(response)
        })
    }).await?;
    Ok(())
}

fn make_portfolio_performance(user_name: &str) -> Result<Vec<HistoryData>, Box<dyn Error>> {
    let conn = establish_connection();
    let user = load_user(&conn, user_name)?;
    let user_portfolio_history = load_user_portfolio_history(&conn, &user, Option::from(5))?;

    let portfolio = load_users_portfolio(&conn, &user)?;
    let current_value = calculate_portfolio_value(&conn, &user, &portfolio)?;

    let mut history_data: Vec<HistoryData> = vec![
        HistoryData{
            date: Utc::now().date().naive_utc(),
            value: current_value,
            difference: current_value - user_portfolio_history.first().unwrap().value
        }
    ];

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

    Ok(history_data)
}

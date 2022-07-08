use std::env;

use serenity::{
    async_trait,
    framework::{
        standard::macros::group,
        StandardFramework,
    },
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
};
use tracing::{error, info};

use commands::{
    buy::*,
    buy_all::*,
    help::*,
    register::*,
    sell::*,
    market::*,
    portfolio::*,
    market_lock::*,
    team_history::*,
    portfolio_history::*,
    leaderboard::*,
    schedule::*,
    weekly_report::*,
    sell_all::*,
    ping::*,
    portfolio_history_graph::*,
    team_history_graph::*,
    leaderboard_graph::*,
    market_cap::*,
    suggest::*,
    suggest_affordable::*,
    update_alias::*,
};

use lol_stocks_core::{
    seed,
    database::migrations::run_migrations,
};

mod commands;
mod helpers;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        let _ = &__arg1;
        info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        let _ = &__arg1;
        info!("Resumed");
    }
}

#[group]
#[commands(help, register, buy, sell, market, portfolio, market_lock, ping, team_history, portfolio_history, leaderboard, schedule, weekly_report, sell_all, buy_all, portfolio_history_graph, team_history_graph, leaderboard_graph, market_cap, suggest, suggest_affordable, update_alias)]
struct General;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    run_migrations();

    seed::add_bot_user();

    println!("Discord Bot Running");
    let framework = StandardFramework::new()
        .configure(|c| c
        .prefix("!!"))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
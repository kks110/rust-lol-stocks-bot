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
    help::*,
    register::*,
    sell::*,
    view_market::*,
    view_portfolio::*,
    db_lock::*,
};

use lol_stocks_core::database::migrations::run_migrations;

mod commands;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[group]
#[commands(help, register, buy, sell, view_market, view_portfolio, db_lock)]
struct General;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    run_migrations();

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
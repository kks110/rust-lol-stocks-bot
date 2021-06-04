mod commands;
mod models;
mod elo;
mod database;
mod schema;

#[macro_use]
extern crate diesel;

use std::env;

use serenity::{
    async_trait,
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
    framework::{
        StandardFramework,
        standard::macros::group,
    },
};

use tracing::{error, info};

use commands::{
    help::*,
    register::*,
    record_match::*,
    buy::*,
    sell::*,
    view_market::*,
    view_portfolio::*,
    sale_lock::*,
    sale_unlock::*,
};

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
#[commands(help, register, record_match, buy, sell, view_market, view_portfolio, sale_lock, sale_unlock)]
struct General;

#[tokio::main]
async fn main() {
    println!("Service Running");
    dotenv::dotenv().expect("Failed to load .env file");

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

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
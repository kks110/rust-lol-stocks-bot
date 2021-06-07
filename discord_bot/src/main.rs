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
    record_match::*,
    register::*,
    sale_lock::*,
    sale_unlock::*,
    sell::*,
    view_market::*,
    view_portfolio::*,
};

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
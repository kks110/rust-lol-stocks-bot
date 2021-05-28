mod commands;
mod models;
mod file_io;
mod elo;
mod database;
mod schema;

#[macro_use]
extern crate diesel;

use std::{
    collections::HashSet,
    env,
};

use serenity::{
    async_trait,
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
    framework::{
        StandardFramework,
        standard::macros::group,
    },
    http::Http,
};

use tracing::{error, info};

use commands::{
    help::*,
    register::*,
    // record_match::*,
};

use file_io::initialise::initialise;

use database::connection::establish_connection;

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
#[commands(help, register)]
struct General;

#[tokio::main]
async fn main() {
    println!("Service starting");
    match initialise() {
        Err(e) => panic!("Error creating files: {}", e),
        Ok(_ok) =>()
    }

    println!("Service Running");
    dotenv::dotenv().expect("Failed to load .env file");

    let connection = establish_connection();
    let team_name = "FNC";
    let elo = 500;
    let new_elo = 510;

    file_io::teams::load_teams();
    file_io::teams::create_team(&connection, &team_name, &elo);
    file_io::teams::load_teams();
    file_io::teams::update_team(&connection, &team_name, new_elo);
    file_io::teams::load_teams();


    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let http = Http::new_with_token(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c
            .owners(owners)
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
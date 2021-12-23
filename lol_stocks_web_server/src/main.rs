use actix_web::{App, HttpServer};
mod models;
mod endpoints;

use lol_stocks_core::database::migrations::run_migrations;
use endpoints::{
    padlock::padlock,
    register_games::{
        register_game,
        register_games
    },
    register_teams::register_teams,
    teams::teams,
    take_team_snapshot::take_snapshot,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    run_migrations();

    println!("Webserver Running on 127.0.0.1:8181");
    println!("Webserver Running on 0.0.0.0:8080");
    HttpServer::new(|| {
        App::new()
            .service(register_game)
            .service(register_games)
            .service(register_teams)
            .service(padlock)
            .service(teams)
            .service(take_snapshot)
    })
        .bind("127.0.0.1:8181")?
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

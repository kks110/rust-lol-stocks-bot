use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
mod models;
mod endpoints;

use models::{
    game::Games,
    key::Key,
};

use lol_stocks_core::database::migrations::run_migrations;

#[post("/register_match")]
async fn register_matches(game_list: web::Json<Games>) -> impl Responder {
    endpoints::register_match::register_matches(game_list.into_inner());
    println!("Matches logged");
    HttpResponse::Ok().body("")
}

#[post("/padlock")]
async fn padlock(key: web::Json<Key>) -> impl Responder {
    endpoints::padlock::padlock(key.unlock);
    let mut response = String::from("Database has been ");
    response.push_str(
        if key.unlock {
            "unlocked"
        } else {
            "locked"
        }
    );
    println!("{}", response);
    HttpResponse::Ok().body(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    run_migrations();

    println!("Webserver Running on 127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .service(register_matches)
            .service(padlock)
    })
        .bind("127.0.0.1:8181")?
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

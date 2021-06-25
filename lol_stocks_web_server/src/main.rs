use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
mod models;
mod endpoints;
use actix_web_static_files;
use tera::{Tera, Context};

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

use lol_stocks_core::database::migrations::run_migrations;
use endpoints::{
    padlock::padlock,
    register_match::register_matches,
    register_teams::register_teams,
    index::index,
};
use crate::models::app_data::AppData;

#[get("/teams/graphs")]
async fn render_tmpl(data: web::Data<AppData>, req:HttpRequest) -> impl Responder {


    let name = req.match_info().get("name").unwrap();
    let mut ctx = Context::new();
    ctx.insert("name", name);
    ctx.insert("products", &vec![1,2,3]);
    let rendered = data.tera.render("loop.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    run_migrations();

    println!("Webserver Running on 127.0.0.1:8181");
    println!("Webserver Running on 0.0.0.0:8080");
    HttpServer::new(|| {
        let generated = generate();
        let tera =
            Tera::new(
                concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")
            ).unwrap();
        App::new()
            .service(actix_web_static_files::ResourceFiles::new(
                "/static", generated,
            ))
            .data(AppData {tera: tera})
            .service(index)
            .service(register_matches)
            .service(register_teams)
            .service(padlock)
    })
        .bind("127.0.0.1:8181")?
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

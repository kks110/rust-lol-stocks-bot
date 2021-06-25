use actix_web::{get, web, Responder, HttpResponse};
use tera::Context;
use crate::models::app_data::AppData;

#[get("/index")]
pub async fn index(data: web::Data<AppData>) -> impl Responder {
    let ctx = Context::new();
    let rendered = data.tera.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}
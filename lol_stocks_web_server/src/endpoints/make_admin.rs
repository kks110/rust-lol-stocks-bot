use std::env;
use lol_stocks_core::{
    database::{
        connection::establish_connection,
        users::make_user_admin
    },
};
use actix_web::{post, web, Responder, HttpResponse, HttpRequest};
use crate::models::user::User;

#[post("/make_admin")]
pub async fn make_admin(req: HttpRequest, user: web::Json<User>) -> impl Responder {
    let conn = establish_connection();

    let key = retrieve_key_from_header(&req);
    let web_auth_key = env::var("WEB_AUTH_KEY").unwrap_or_else(|_| "".to_string());

    if key == web_auth_key {
        match make_user_admin(&conn, &user.user_name) {
            Ok(_) => HttpResponse::Ok().body(String::from("User made admin")),
            Err(e) => HttpResponse::NotModified().body(e.to_string())
        }
    } else {
        HttpResponse::Unauthorized().body("Unauthorised".to_string())
    }
}

fn retrieve_key_from_header(req: &HttpRequest) -> String {
    if let Some(key) = get_key(req) {
        key.to_string()
    } else {
        "".to_string()
    }
}

fn get_key(req: &HttpRequest) -> Option<&str> {
    req.headers().get("web-auth-key")?.to_str().ok()
}

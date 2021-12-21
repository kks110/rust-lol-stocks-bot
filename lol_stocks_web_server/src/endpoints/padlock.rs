use lol_stocks_core::{
    database::{
        connection::establish_connection,
        locks::{ unlock_database, lock_database }
    },
};
use actix_web::{post, web, Responder, HttpResponse};
use crate::models::key::Key;

#[post("/padlock")]
pub async fn padlock(key: web::Json<Key>) -> impl Responder {
    let response: String;

    let conn = establish_connection();

    if key.unlock {
        match unlock_database(&conn) {
            Ok(_) => response = String::from("Database has been unlocked"),
            Err(e) => response = e.to_string()
        };
    } else {
        match lock_database(&conn) {
            Ok(_) => response = String::from("Database has been locked"),
            Err(e) => response = e.to_string()
        };
    }

    println!("{}", response);
    HttpResponse::Ok().body(response)
}

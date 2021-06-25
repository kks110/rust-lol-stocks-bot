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
    use_padlock(key.unlock);
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

fn use_padlock(unlock: bool) {
    let conn = establish_connection();

    if unlock {
        unlock_database(&conn);
    } else {
        lock_database(&conn);
    }
}
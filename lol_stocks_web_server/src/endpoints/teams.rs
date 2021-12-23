use lol_stocks_core::{
    database::{
        connection::establish_connection,
        teams::load_teams
    },
};
use actix_web::{get, Responder, HttpResponse};

#[get("/teams")]
pub async fn teams() -> impl Responder {
    let conn = establish_connection();
    match load_teams(&conn) {
        Ok(teams) => {
            HttpResponse::Ok()
                .content_type("application/json")
                .json(teams)
        },
        Err(e) => {
            HttpResponse::InternalServerError()
                .body(e.to_string())
        }
    }
}

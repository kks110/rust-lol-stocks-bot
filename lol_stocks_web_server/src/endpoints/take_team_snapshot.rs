use lol_stocks_core::{
    histories::take_history_snapshot,
};
use actix_web::{post, Responder, HttpResponse};

#[post("/take_snapshot")]
pub async fn take_snapshot() -> impl Responder {
    match take_history_snapshot() {
        Ok(_) => {
            HttpResponse::Ok().body("")
        },
        Err(e) => {
            HttpResponse::InternalServerError()
                .body(e.to_string())
        }
    }
}

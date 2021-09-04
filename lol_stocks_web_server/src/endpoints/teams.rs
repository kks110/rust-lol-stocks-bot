use lol_stocks_core::{
    database::{
        connection::establish_connection,
        teams::{
            load_teams,
            load_team_by_id
        },
    },
};
use actix_web::{get, web, Responder, HttpResponse};

#[get("/teams")]
pub async fn list() -> impl Responder {
    let conn = establish_connection();
    let teams = load_teams(&conn);
    HttpResponse::Ok().json(teams)
}

#[get("/teams/{id}")]
pub async fn get(id: web::Path<i32>) -> impl Responder {
    let conn = establish_connection();
    let teams = load_team_by_id(&conn, &id);
    HttpResponse::Ok().json(teams)

}

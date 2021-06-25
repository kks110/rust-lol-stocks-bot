use crate::models::team::Teams;
use lol_stocks_core::{
    database::{
        connection::establish_connection,
        teams::{ create_team },
    },
};
use actix_web::{post, web, Responder, HttpResponse};

#[post("/register_teams")]
pub async fn register_teams(team_list: web::Json<Teams>) -> impl Responder {
    register(team_list.into_inner());
    println!("Teams added to DB");
    HttpResponse::Ok().body("")
}

fn register(teams: Teams) {
    let conn = establish_connection();

    for team in teams.teams {
        create_team(&conn, &team);
    }
}

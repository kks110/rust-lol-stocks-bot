use crate::models::game::Games;
use lol_stocks_core::{
    database::{
        connection::establish_connection,
        teams::{ load_team, update_team },
    },
    elo::calculate_elo,
    histories::take_history_snapshot
};
use actix_web::{post, web, Responder, HttpResponse};


#[post("/register_matches")]
pub async fn register_matches(game_list: web::Json<Games>) -> impl Responder {
    register(game_list.into_inner());
    println!("Matches logged");
    HttpResponse::Ok().body("")
}

fn register(games: Games) {
    take_history_snapshot();

    for game in games.matches {
        let winner =  game.winner;
        let loser = game.looser;

        let conn = establish_connection();

        let winning_team = load_team(&conn, &winner);
        let losing_team = load_team(&conn, &loser);

        let (winning_elo, losing_elo) = calculate_elo(winning_team.elo.clone(), losing_team.elo.clone());
        update_team(&conn, &winning_team.name, winning_elo);
        update_team(&conn, &losing_team.name, losing_elo);
    }
}

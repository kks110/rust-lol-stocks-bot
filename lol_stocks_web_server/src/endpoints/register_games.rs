use std::error::Error;
use crate::models::game::{Games, Game};
use lol_stocks_core::{
    database::{
        connection::establish_connection,
        teams::{ load_team, update_team },
    },
    elo::calculate_elo,
};
use actix_web::{post, web, Responder, HttpResponse};


#[post("/register_game")]
pub async fn register_game(game: web::Json<Game>) -> impl Responder {
    match register_one_game(&game.winner, &game.looser) {
        Ok(_) => {
            println!("Matches logged");
            HttpResponse::Ok().body("")
        },
        Err(e) => {
            println!("An error occurred: {}", e.to_string());
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }

}

#[post("/register_games")]
pub async fn register_games(game_list: web::Json<Games>) -> impl Responder {
    match register_game_list(game_list.into_inner()) {
        Ok(_) => {
            println!("Matches logged");
            HttpResponse::Ok().body("")
        },
        Err(e) => {
            println!("An error occurred: {}", e.to_string());
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }

}

fn register_one_game(winner: &str, looser: &str) -> Result<(), Box<dyn Error>> {
    let conn = establish_connection();

    let winning_team = load_team(&conn, winner)?;
    let losing_team = load_team(&conn, looser)?;

    let (winning_elo, losing_elo) = calculate_elo(winning_team.elo, losing_team.elo);
    update_team(&conn, &winning_team.name, winning_elo)?;
    update_team(&conn, &losing_team.name, losing_elo)?;

    Ok(())
}

fn register_game_list(games: Games) -> Result<(), Box<dyn Error>> {
    for game in games.matches {
        let winner =  game.winner;
        let loser = game.looser;

        let conn = establish_connection();

        let winning_team = load_team(&conn, &winner)?;
        let losing_team = load_team(&conn, &loser)?;

        let (winning_elo, losing_elo) = calculate_elo(winning_team.elo, losing_team.elo);
        update_team(&conn, &winning_team.name, winning_elo)?;
        update_team(&conn, &losing_team.name, losing_elo)?;
    }

    Ok(())
}

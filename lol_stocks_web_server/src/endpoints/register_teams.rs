use crate::models::team::Teams;
use lol_stocks_core::{
    database::{
        connection::establish_connection,
        teams::{ create_team },
    },
};

pub fn register_teams(teams: Teams) {
    let conn = establish_connection();

    for team in teams.teams {
        create_team(&conn, &team);
    }
}

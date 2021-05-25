use crate::file_io::data::location_of;
use crate::models::teams::*;
use crate::elo::elo;

use std::fs;
use std::io::{Error, ErrorKind};

pub fn update_elo(winning_team: &str, loosing_team: &str) -> Result<(), Error> {
    let teams = load_teams()?;
    let mut team_winner: Team = Team{name: String::from(""), elo: 0};
    let mut team_looser: Team = Team{name: String::from(""), elo: 0};
    for team in teams.teams.clone() {
        if team.name == winning_team {
            team_winner = team.clone()
        }
        if team.name == loosing_team {
            team_looser = team.clone()
        }
    }
    if team_winner.name == String::from("") || team_looser.name == String::from("") {
        return Err(Error::new(ErrorKind::Other, "oh no!"))
    }

    let (new_winning_elo, new_loosing_elo) = elo::calculate_elo(&team_winner, &team_looser);

    let mut new_team_list: TeamsList = TeamsList{ teams: Vec::new()};
    for mut team in teams.teams.clone() {
        if team.name == winning_team {
            team.elo = new_winning_elo
        }
        if team.name == loosing_team {
            team.elo = new_loosing_elo
        }
        new_team_list.teams.push(team.to_owned());
    }

    save_teams(new_team_list)?;
    Ok(())
}

fn load_teams() -> Result<TeamsList, Error> {
    let teams_location = location_of("data/teams.json");
    let file = fs::read_to_string(teams_location)?;

    let team_list = serde_json::from_str(&file)?;
    Ok(team_list)
}

fn save_teams(team_list: TeamsList) -> Result<(), Error> {
    let data_to_save = serde_json::to_string(&team_list)?;
    let teams_location = location_of("data/teams.json");
    fs::write(teams_location, data_to_save)?;
    Ok(())
}

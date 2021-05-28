// use crate::file_io::data::location_of;
// use crate::models::teams::*;
// use crate::elo::elo;
//
// use std::fs;
// use std::io::{Error, ErrorKind};
//
// pub fn update_elo(winning_team: &str, loosing_team: &str) -> Result<(), Error> {
//     let teams = load_teams()?;
//     let mut team_winner: Team = Team{name: String::from(""), elo: 0};
//     let mut team_looser: Team = Team{name: String::from(""), elo: 0};
//     for team in teams.teams.clone() {
//         if team.name == winning_team {
//             team_winner = team.clone()
//         }
//         if team.name == loosing_team {
//             team_looser = team.clone()
//         }
//     }
//     if team_winner.name == String::from("") || team_looser.name == String::from("") {
//         return Err(Error::new(ErrorKind::Other, "oh no!"))
//     }
//
//     let (new_winning_elo, new_loosing_elo) = elo::calculate_elo(&team_winner, &team_looser);
//
//     let mut new_team_list: TeamsList = TeamsList{ teams: Vec::new()};
//     for mut team in teams.teams.clone() {
//         if team.name == winning_team {
//             team.elo = new_winning_elo
//         }
//         if team.name == loosing_team {
//             team.elo = new_loosing_elo
//         }
//         new_team_list.teams.push(team.to_owned());
//     }
//
//     save_teams(new_team_list)?;
//     Ok(())
// }
//
// fn load_teams() -> Result<TeamsList, Error> {
//     let teams_location = location_of("data/teams.json");
//     let file = fs::read_to_string(teams_location)?;
//
//     let team_list = serde_json::from_str(&file)?;
//     Ok(team_list)
// }
//
// fn save_teams(team_list: TeamsList) -> Result<(), Error> {
//     let data_to_save = serde_json::to_string(&team_list)?;
//     let teams_location = location_of("data/teams.json");
//     fs::write(teams_location, data_to_save)?;
//     Ok(())
// }



use crate::database::connection;
use crate::diesel::prelude::*;
use crate::models::team::{Team, NewTeam};


pub fn load_teams() {
    use crate::schema::teams::dsl::*;

    let connection = connection::establish_connection();
    let results = teams.filter(name.eq("FNC"))
        .limit(5)
        .load::<Team>(&connection)
        .expect("Error loading teams");

    println!("Displaying {} teams", results.len());
    for team in results {
        println!("{}", team.name);
        println!("----------\n");
        println!("{}", team.elo);
    }
}

pub fn create_team<'a>(conn: &PgConnection, name: &'a str, elo: &'a i32) -> Team {
    use crate::schema::teams;

    let new_team = NewTeam {
        name,
        elo,
    };

    diesel::insert_into(teams::table)
        .values(&new_team)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn update_team<'a>(conn: &PgConnection, team_name: &str, new_elo: i32) -> Team {
    use crate::schema::teams::dsl::*;

    let team = diesel::update(teams.filter(name.eq(team_name)))
        .set(elo.eq(new_elo))
        .get_result::<Team>(conn)
        .expect(&format!("Unable to find team {}", team_name));
    println!("Updated team {}. New ELO is {}", team.name, team.elo);
    return team;
}
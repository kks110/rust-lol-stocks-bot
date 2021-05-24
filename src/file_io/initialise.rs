use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::{ Path, PathBuf };
use crate::file_io::data::location_of;

pub fn initialise() -> std::io::Result<()> {
    println!("Checking data files");

    let data_location = location_of("data");
    let users_location = location_of("data/users.json");
    let teams_location = location_of("data/teams.json");

    if !location_exists(&data_location) {
        create_folder(&data_location)?
    }

    if !location_exists(&users_location) {
        create_users_file(&users_location)?
    }

    if !location_exists(&teams_location) {
        create_teams_file(&teams_location)?
    }

    Ok(())
}

fn location_exists(path: &PathBuf) -> bool {
    Path::new(&path).exists()
}

fn create_users_file(path: &PathBuf) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(b"{ \"users\": []}")?;
    Ok(())
}

fn create_teams_file(path: &PathBuf) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(b"{ \"teams\": []}")?;
    Ok(())
}

fn create_folder(path: &PathBuf) -> std::io::Result<()> {
    fs::create_dir(path)?;
    Ok(())
}

use diesel_migrations::embed_migrations;

embed_migrations!();

use crate::database::connection::establish_connection;

pub fn run_migrations() {
    println!("Running DB migrations");
    let conn = establish_connection();
    match embedded_migrations::run(&conn) {
        Ok(_) => println!("Migrations run successfully"),
        Err(e) => println!("Error running migrations: {}", e)
    }
}

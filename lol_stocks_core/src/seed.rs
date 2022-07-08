use crate::database::connection::establish_connection;
use crate::database::users;

pub fn add_bot_user() {
    let conn = establish_connection();
    match users::create_user(&conn, "StockBot", &0, None) {
        Ok(_) => { println!("Created StockBot user") }
        Err(e) => {
            if e.to_string() == *"duplicate key value violates unique constraint \"unique_user_name\"".to_string() {
                println!("StockBot user already created.")
            } else {
                panic!("Error creating StockBot user")
            }
        }
    }
}


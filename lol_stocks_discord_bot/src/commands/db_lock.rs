use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

use lol_stocks_core::database::{
    connection::establish_connection,
    locks::{lock_database, unlock_database, load_lock},
    users::load_user,
};
use lol_stocks_core::models::lock::Lock;


#[command]
pub async fn db_lock(ctx: &Context, msg: &Message) -> CommandResult {
    let conn = establish_connection();
    let user = load_user(&conn, &msg.author.name);
    let db_lock: Option<Lock>;
    let mut response= String::from("");

    match load_lock(&conn) {
        Ok(l) => db_lock = Some(l),
        Err(e) => {
            response.push_str(&e);
            db_lock = None
        }
    }

    if db_lock.is_some() {
        if user.admin {
            if db_lock.unwrap().locked {
                println!("Unlocking database");
                match unlock_database(&conn) {
                    Ok(_) => response = String::from("Market is open! Happy Shopping"),
                    Err(e) => response = e
                };

            } else {
                println!("Locking database");
                match lock_database(&conn) {
                    Ok(_) => response = String::from("Market is closed! Time to watch some games"),
                    Err(e) => response = e
                };
            }
        } else {
            response = String::from("Only admins can do this!")
        }
    }

    msg.channel_id.say(&ctx.http, response).await?;

    Ok(())
}

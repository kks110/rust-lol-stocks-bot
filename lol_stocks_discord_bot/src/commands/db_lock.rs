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


#[command]
pub async fn db_lock(ctx: &Context, msg: &Message) -> CommandResult {
    let conn = establish_connection();
    let lock = load_lock(&conn);
    let user = load_user(&conn, &msg.author.name);
    let response: String;

    if user.admin {
        if lock.locked {
            println!("Unlocking database");
            unlock_database(&conn);
            response = String::from("Database unlocked")
        } else {
            println!("Locking database");
            lock_database(&conn);
            response = String::from("Database locked")
        }
    } else {
        response = String::from("Only admins can do this!")
    }

    msg.channel_id.say(&ctx.http, response).await?;

    Ok(())
}

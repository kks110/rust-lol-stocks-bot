use lol_stocks_core::{
    database::{
        connection::establish_connection,
        locks::{ unlock_database, lock_database }
    },
};

pub fn padlock(unlock: bool) {
    let conn = establish_connection();

    if unlock {
        unlock_database(&conn);
    } else {
        lock_database(&conn);
    }
}
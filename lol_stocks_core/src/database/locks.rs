use diesel::prelude::*;
use std::error::Error;
use crate::models::lock::{Lock};

pub fn load_lock(conn: &PgConnection) -> Result<Lock, Box<dyn Error>> {
    use crate::schema::locks::dsl::*;

    Ok(locks.first(conn)?)
}

pub fn lock_database(conn: &PgConnection) -> Result<Lock, Box<dyn Error>> {
    use crate::schema::locks::dsl::*;
    let lock = load_lock(conn)?;

    Ok(diesel::update(locks.filter(id.eq(lock.id)))
        .set(locked.eq(true))
        .get_result::<Lock>(conn)?
    )
}

pub fn unlock_database(conn: &PgConnection) -> Result<Lock, Box<dyn Error>> {
    use crate::schema::locks::dsl::*;
    let lock = load_lock(conn)?;

    Ok(diesel::update(locks.filter(id.eq(lock.id)))
        .set(locked.eq(false))
        .get_result::<Lock>(conn)?
    )
}

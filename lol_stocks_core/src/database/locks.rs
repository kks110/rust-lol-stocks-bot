use diesel::prelude::*;
use crate::models::lock::{Lock};

pub fn load_lock(conn: &PgConnection) -> Result<Lock, String> {
    use crate::schema::locks::dsl::*;

    match locks.first(conn) {
        Ok(lock) => Ok(lock),
        Err(e) => Err(e.to_string())
    }
}

pub fn lock_database<'a>(conn: &PgConnection) -> Result<Lock, String> {
    use crate::schema::locks::dsl::*;
    let lock =  match load_lock(&conn) {
        Ok(l) => l,
        Err(e) => return Err(e)
    };

    return match diesel::update(locks.filter(id.eq(lock.id)))
        .set(locked.eq(true))
        .get_result::<Lock>(conn) {
        Ok(l) => Ok(l),
        Err(e) => Err(e.to_string())
    };
}

pub fn unlock_database<'a>(conn: &PgConnection) -> Result<Lock, String> {
    use crate::schema::locks::dsl::*;
    let lock = match load_lock(&conn) {
        Ok(l) => l,
        Err(e) => return Err(e)
    };

    return match diesel::update(locks.filter(id.eq(lock.id)))
        .set(locked.eq(false))
        .get_result::<Lock>(conn) {
        Ok(l) => Ok(l),
        Err(e) => Err(e.to_string())
    };
}

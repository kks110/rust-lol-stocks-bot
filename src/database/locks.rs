use crate::diesel::prelude::*;
use crate::models::lock::{Lock};

pub fn load_lock(conn: &PgConnection) -> Lock {
    use crate::schema::locks::dsl::*;

    locks.first(conn)
        .expect("Error loading lock")
}

pub fn lock_database<'a>(conn: &PgConnection) -> Lock {
    use crate::schema::locks::dsl::*;
    let lock = load_lock(&conn);

    let updated_lock = diesel::update(locks.filter(id.eq(lock.id)))
        .set(locked.eq(true))
        .get_result::<Lock>(conn)
        .expect(&format!("Unable to find lock"));
    return updated_lock;
}

pub fn unlock_database<'a>(conn: &PgConnection) -> Lock {
    use crate::schema::locks::dsl::*;
    let lock = load_lock(&conn);

    let updated_lock = diesel::update(locks.filter(id.eq(lock.id)))
        .set(locked.eq(false))
        .get_result::<Lock>(conn)
        .expect(&format!("Unable to find lock"));
    return updated_lock;
}

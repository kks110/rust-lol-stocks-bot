use crate::schema::locks;

#[derive(Identifiable, Queryable)]
pub struct Lock {
    pub id: i32,
    pub locked: bool,
}

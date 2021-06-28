use crate::schema::leagues;

#[derive(Identifiable, Queryable)]
pub struct League {
    pub id: i32,
    pub name: String
}

#[derive(Insertable)]
#[table_name="leagues"]
pub struct NewLeague<'a> {
    pub name: &'a str
}

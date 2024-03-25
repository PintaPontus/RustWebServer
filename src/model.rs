use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use crate::schema::items;

#[derive(Insertable)]
#[table_name = "items"]
pub struct NewItem <'a> {
    pub nome: &'a str,
    pub valore: i32,
}

#[derive(Debug, Selectable, Queryable, AsChangeset)]
pub struct Items<'a> {
    pub id: i32,
    pub nome: &'a str,
    pub valore: i32,
}

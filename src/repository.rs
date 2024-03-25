use crate::db_connection::establish_connection;
use crate::model::{Items, NewItem};
use crate::schema::items::dsl::items;
use diesel::{QuerySource, RunQueryDsl};

pub fn get() -> Vec<Items<'static>> {
    let mut connection = establish_connection();
    items.default_selection().load::<Items<'static>>(&mut connection).expect("Error on select")
}

pub fn post() -> usize {
    let mut connection = establish_connection();
    let new_item = NewItem {
        nome: "cieo",
        valore: 1,
    };
    diesel::insert_into(items)
        .values(&new_item)
        .execute(&mut connection)
        .expect("Error on insert")
}

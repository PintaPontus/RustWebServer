#[macro_use]
extern crate rocket;

use crate::crud_controller::{delete, get, post, put};

mod crud_controller;
mod db_connection;
mod model;
mod repository;
mod schema;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get, post, put, delete])
}

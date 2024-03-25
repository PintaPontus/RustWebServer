mod crud_controller;
mod db_connection;
mod schema;
mod repository;
mod model;

#[macro_use]
extern crate rocket;

use crate::crud_controller::{delete, get, post, put};

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get, post, put, delete])
}

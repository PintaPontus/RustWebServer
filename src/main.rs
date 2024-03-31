use axum::{routing::get, Router};

use crate::crud_controller::*;

mod crud_controller;
mod db_connection;
mod model;
mod repository;
mod schema;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/users", get(all_users).post(new_user))
        .route("/users/:user_id", get(one_user));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

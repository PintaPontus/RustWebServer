use axum::extract::Path;
use axum::Json;

use crate::model::NewUser;
use crate::repository::{add_user, get_all_users, get_one_user};

pub async fn hello_world() -> &'static str {
    "Hello World!"
}

pub async fn all_users() -> String {
    get_all_users()
        .await
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

pub async fn one_user(Path(user_id): Path<i64>) -> String {
    get_one_user(user_id).await.to_string()
}

pub async fn new_user(Json(payload): Json<NewUser>) {
    add_user(payload).await;
}

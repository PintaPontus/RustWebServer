use diesel::{insert_into, QueryDsl};
use diesel_async::RunQueryDsl;

use crate::db_connection::get_connection;
use crate::model::{NewUser, User};
use crate::schema::users;

pub async fn get_all_users() -> Vec<User> {
    users::table
        // .select(User::as_select())
        .load(&mut get_connection().await)
        .await
        .unwrap()
}

pub async fn get_one_user(user_id: i64) -> User {
    users::table
        .find(user_id)
        .first(&mut get_connection().await)
        .await
        .unwrap()
}

pub async fn add_user(user: NewUser) {
    insert_into(users::table)
        .values(user)
        .execute(&mut get_connection().await)
        .await
        .unwrap();
}

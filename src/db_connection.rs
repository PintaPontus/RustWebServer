use std::env;

use diesel_async::{AsyncConnection, AsyncPgConnection};

// TODO: add connection pool
pub async fn get_connection() -> AsyncPgConnection {
    return AsyncPgConnection::establish(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
}

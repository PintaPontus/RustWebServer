// use tokio_postgres::Row;
//
// use crate::db_connection::get_client;
//
// pub async fn get_all_users() -> Vec<Row> {
//     let client = get_client().await;
//     match client {
//         Ok(mut client) => {
//             client.query("SELECT * FROM users",&[]).unwrap()
//         }
//         Err(error) => {
//             println!("There is an error {error}");
//             Vec::new()
//         }
//     }
// }
//
//
// pub fn post() -> &'static str {
//     "POST"
// }
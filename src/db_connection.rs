// use tokio_postgres::{Client, Error, NoTls};
//
// pub async fn get_client() -> Client {
//     let connection_string = "host=localhost user=postgres dbname=postgres password=password";
//     let (client, connection) = tokio_postgres::connect(connection_string, NoTls).await.unwrap();
//
//     // Spawn a task to handle the connection
//     tokio::spawn(async move {
//         if let Err(e) = connection.await {
//             eprintln!("Connection error: {}", e);
//         }
//     });
//
//     return client;
// }

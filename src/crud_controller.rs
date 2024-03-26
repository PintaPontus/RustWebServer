use tokio_postgres::NoTls;

#[get("/")]
pub async fn get() -> String {
    // Set up the connection URL
    let conn_string = "host=localhost user=postgres password=password dbname=postgres";

    // Connect to the database
    let (client, connection) = tokio_postgres::connect(conn_string, NoTls).await.unwrap();

    // Spawn a task to handle the connection
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    // Assuming you have a single row in your_table
    match client.query("SELECT * FROM users", &[]).await {
        Ok(rows) => rows
            .iter()
            .map(|row| -> String { row.get("name") })
            .collect::<Vec<_>>()
            .join(";\n"),
        Err(e) => {
            eprintln!("query error: {}", e);
            e.to_string()
        }
    }
}

#[post("/")]
pub fn post() -> &'static str {
    "POST"
}

#[put("/")]
pub fn put() -> &'static str {
    "PUT"
}

#[delete("/")]
pub fn delete() -> &'static str {
    "DELETE"
}

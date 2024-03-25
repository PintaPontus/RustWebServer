use diesel::prelude::*;

pub fn establish_connection() -> PgConnection {
 let database_url = "";
    PgConnection::establish(database_url).expect("Errore")
}


use std::fmt::{Display, Formatter, Result};

use diesel::{Insertable, Queryable, Selectable};
use serde::Deserialize;

use crate::schema::users;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = users)]
#[allow(dead_code)]
pub struct User {
    id: i64,
    name: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

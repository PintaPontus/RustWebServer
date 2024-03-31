use diesel::table;

table! {
    users {
        id -> BigInt,
        name -> VarChar,
    }
}

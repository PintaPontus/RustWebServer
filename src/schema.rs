use diesel::table;
table!{
    items (id){
        id -> Int4,
        nome -> VarChar,
        valore -> Int4,
    }
}
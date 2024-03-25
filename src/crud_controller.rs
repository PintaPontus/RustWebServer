
#[get("/")]
pub fn get() -> &'static str {
    "GET"
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
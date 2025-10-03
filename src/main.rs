mod auth;

#[macro_use] extern crate rocket;

use rocket::serde::json::{Value};
use serde_json::json;
use rocket::response::status;

#[get("/members")]
fn get_members(_auth: auth::BasicAuth) -> Value {
    json!([{"id":1, "name":"Alireza"},{"id":2, "name":"John"}])
}

#[get("/members/<id>")]
fn get_member(id:i32, _auth: auth::BasicAuth) -> Value {
    json!([{"id":id, "name":"Alireza"}])
}

#[post("/members", format="application/json")]
fn create_member(_auth: auth::BasicAuth) -> Value {
    json!([{"id":3, "name":"Unknown"}])
}

#[put("/members/<id>", format="application/json")]
fn update_member(id:i32, _auth: auth::BasicAuth) -> Value {
    json!([{"id":id, "name":"Unknown"}])
}

#[delete("/members/<id>")]
fn delete_member(id:i32, _auth: auth::BasicAuth) -> status::NoContent {
    status::NoContent
}

#[get("/stats")]
fn stats() -> Value {
    json!({"status": "ok"})
}

#[catch(404)]
fn not_found() -> Value {
    json!({"status": "Not found"})
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![
            stats,
            create_member,
            update_member,
            delete_member,
            get_members,
            get_member,
        ])
        .register("/", catchers![not_found])
        .launch().await;
}

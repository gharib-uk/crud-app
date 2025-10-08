#[macro_use] extern crate rocket;

mod auth;
mod schema;
mod models;
mod repositories;

use auth::BasicAuth;
use diesel::result::Error::NotFound;
use rocket::{Rocket, Build};
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::serde::json::{Value, json, Json};
use rocket::response::status::{self, Custom};
use rocket_sync_db_pools::database;
use repositories::MemberRepository;
use crate::models::{Member, NewMember};

#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

#[get("/members")]
async fn get_members(_auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        MemberRepository::find_multiple(c, 100)
            .map(|members| json!(members))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}
#[get("/members/<id>")]
async fn view_member(id: i32, _auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        MemberRepository::find(c, id)
            .map(|member| json!(member))
            .map_err(|e|
                match e {
                    NotFound => Custom(Status::NotFound, json!(e.to_string())),
                    _ => Custom(Status::InternalServerError, json!(e.to_string()))
                }
            )
    }).await
}
#[post("/members", format = "json", data = "<new_member>")]
async fn create_member(_auth: BasicAuth, db: DbConn, new_member: Json<NewMember>) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        MemberRepository::create(c, new_member.into_inner())
            .map(|member| json!(member))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}
#[put("/members/<id>", format = "json", data = "<member>")]
async fn update_member(id: i32, _auth: BasicAuth, db: DbConn, member: Json<Member>) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        MemberRepository::save(c, id, member.into_inner())
            .map(|member| json!(member))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}
#[delete("/members/<id>")]
async fn delete_member(id: i32, _auth: BasicAuth, db: DbConn) -> Result<status::NoContent, Custom<Value>> {
    db.run(move |c| {
        MemberRepository::delete(c, id)
            .map(|_| status::NoContent)
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

async fn run_db_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    DbConn::get_one(&rocket)
        .await
        .expect("Unable to retrieve connection").run(|c| {
        c.run_pending_migrations(MIGRATIONS).expect("Migrations failed");
    })
        .await;

    rocket
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not found!")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![
            get_members,
            view_member,
            create_member,
            update_member,
            delete_member
        ])
        .register("/", catchers![
            not_found
        ])
        .attach(DbConn::fairing())
        .attach(AdHoc::on_ignite("Diesel migrations", run_db_migrations))
        .launch()
        .await;
}
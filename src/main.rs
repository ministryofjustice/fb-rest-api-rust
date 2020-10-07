#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;

use rocket_contrib::json::{Json};

mod db;
mod schema;
mod models;

use models::Service;

#[get("/health")]
fn health() -> &'static str {
    "ok"
}

#[post("/services", format = "json", data = "<service>")]
fn create_service(service: Json<Service>, connection: db::Connection) -> Json<Service> {
    Json(Service::create(service.into_inner(), &connection))
}

#[get("/services/<id>")]
fn find_service(id: i32, connection: db::Connection) -> Json<Service> {
    Json(Service::find_by_id(id, &connection))
}

#[database("db")]
struct DbConn(diesel::PgConnection);

fn main() {
    rocket::ignite()
        .manage(db::connect())
        .mount("/", routes![health, create_service, find_service])
        .launch();
}

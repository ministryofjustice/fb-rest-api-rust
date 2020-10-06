#![feature(proc_macro_hygiene, decl_macro)]

//pub mod schema;
//pub mod models;

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

//use crate::models::Service;
use serde::{Serialize, Deserialize};
use rocket_contrib::json::{Json, JsonValue};

#[get("/health")]
fn health() -> &'static str {
    "ok"
}

#[derive(Debug, Deserialize)]
struct ServiceParam {
    name: String
}

#[post("/services", format = "json", data = "<params>")]
fn create_service(params: Json<ServiceParam>) -> String {
    format!("print test {:?}", params)
}


// #[get("/services/<id>")]
// fn service(id: str) {
// }

#[database("db")]
struct DbConn(diesel::PgConnection);

fn main() {
    rocket::ignite()
        .mount("/", routes![health, create_service])
        .attach(DbConn::fairing())
        .launch();
}

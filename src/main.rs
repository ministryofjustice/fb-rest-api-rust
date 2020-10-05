#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

#[get("/health")]
fn health() -> &'static str {
    "ok"
}

#[database("db")]
struct DbConn(diesel::PgConnection);

fn main() {
    rocket::ignite()
        .mount("/", routes![health])
        .attach(DbConn::fairing())
        .launch();
}
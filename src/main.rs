#![feature(proc_macro_hygiene, decl_macro)]

extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

#[cfg(test)] mod tests;

pub mod cors;
pub mod models;
pub mod routes;
pub mod schema;

use rocket_contrib::databases::diesel::pg;

#[database("app")]
pub struct DbConn(pg::PgConnection);

fn rocket() -> rocket::Rocket
{
    rocket::ignite()
        .mount(
            "/",
            routes![
                routes::index,
                routes::hello,
                routes::create_package,
                routes::list_packages,
                routes::create_group,
                routes::list_groups
            ],
        )
        .attach(DbConn::fairing())
        .attach(cors::CorsFairing)
}

fn main() {
    rocket().launch();
}
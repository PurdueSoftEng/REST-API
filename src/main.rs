#![feature(proc_macro_hygiene, decl_macro)]

extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derives;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate metrics;
extern crate base64;
extern crate zip;
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
                routes::inject_package,
                routes::create_package,
                routes::query_package,
                routes::list_packages,
                routes::get_package,
                routes::delete_package,
                routes::update_package,
                routes::get_rating,
                routes::search_regex,
                routes::get_hist,
                routes::delete_hist,
                routes::auth,
                routes::reset,
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
#[macro_use]
extern crate actix_web;

use std::{env, io};

use actix_web::{middleware, App, HttpServer};

mod end_point;
mod constants;
mod response;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()

            .wrap(middleware::Logger::default())

            .service(end_point::list)
            .service(end_point::get)
            .service(end_point::create)
            .service(end_point::delete)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}

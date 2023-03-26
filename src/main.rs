#[macro_use]
extern crate actix_web;

use std::{env, io};

use actix_web::{middleware, App, HttpServer};

use dotenv::dotenv;
//use listenfd::ListenFd;

mod end_point;
mod constants;
mod response;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    dotenv().ok();

    //let mut listenfd = ListenFd::from_env();
    
    let host = env::var("HOST").expect("Please set host in .env");
    let port = env::var("PORT").expect("Please set port in .env");
    HttpServer::new(|| {
        App::new()

            .wrap(middleware::Logger::default())

            .service(end_point::list)
            .service(end_point::get)
            .service(end_point::create)
            .service(end_point::delete)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

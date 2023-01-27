use std::{env, io};

use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;

use listenfd::ListenFd;

mod end_point;
mod constants;
mod error_handle;
mod response_handle;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(end_point::list)
            .service(end_point::get)
            .service(end_point::create)
            .service(end_point::delete)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = "127.0.0.1";
            let port = "5000";
            server.bind(format!("{}:{}", host, port))?
        }
    };

    server.run().await
}

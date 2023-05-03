use rocket::fairing::{Fairing, Info, Kind};
use rocket::{http::Method, http::Status, Request, Response};

pub struct CorsFairing;

impl Fairing for CorsFairing {
    fn on_response(&self, request: &Request, response: &mut Response) {
        response.set_header(rocket::http::Header::new("Access-Control-Allow-Origin","*",));
        response.set_header(rocket::http::Header::new("Access-Control-Allow-Methods", "POST, GET, DELETE, PUT, OPTIONS"));
        response.set_header(rocket::http::Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(rocket::http::Header::new("Access-Control-Allow-Credentials", "true"));

        if response.status() == Status::NotFound && request.method() == Method::Options {
            response.set_status(Status::NoContent);
        }
    }

    fn info(&self) -> Info {
        Info {
            name: "CORS Fairing",
            kind: Kind::Response,
        }
    }
}
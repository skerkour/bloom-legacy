mod db;
mod server;
mod log;

use actix_web::{server as actix_server, App, HttpRequest, HttpResponse, Result as ActixResult};
use actix::System;
use serde::{Serialize, Deserialize};
use server::middlewares;
use middlewares::logger::GetRequestLogger;
use crate::log::macros::{slog_info, slog_o};

#[derive(Serialize)]
pub struct Response<T: Serialize> {
    data: Option<T>,
    error: Option<ResponseError>
}

#[derive(Serialize)]
pub struct ResponseError{
    message: String,
}

impl<T: Serialize> Response<T> {
    fn data(data: T) -> Response<T> {
        return Response{
            data: Some(data),
            error: None,
            };
    }

    fn error(err: &str) -> Response<T> {
        return Response{
            data: None,
            error: Some(ResponseError{message: err.to_string()}),
        };
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct HelloWorld<'a> {
    pub hello: &'a str,
}

fn index(req: &HttpRequest) -> ActixResult<HttpResponse> {
    let res = HelloWorld{hello: "world"};
    let logger = req.logger();
    slog_info!(logger, "hello world");
    return Ok(HttpResponse::Ok().json(Response::data(res)));
}

fn main() {
    let (_guard, logger) = log::setup_slog();
    let sys = System::new("kernel");

    actix_server::new(|| {
        App::new()
        .middleware(middlewares::request_id::RequestIDHeader)
        .middleware(middlewares::logger::Logger)
        .resource("/", |r| r.method(http::Method::GET).f(index))
        .resource("/hello", |r| r.method(http::Method::GET).f(index))
        .default_resource(|r|
            r.f(|_| HttpResponse::NotFound().json(Response::<String>::error("Not Found")))
        )
    })
    .backlog(8192)
    .bind("127.0.0.1:8000")
    .unwrap()
    .keep_alive(actix_server::KeepAlive::Timeout(60))
    .shutdown_timeout(2)
    .workers(8)
    .start();

    slog_info!(logger, "server started"; slog_o!("port" => "8000"));
    let _ = sys.run();
}

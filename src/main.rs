#[macro_use]
extern crate failure;

mod db;
mod server;
mod log;
mod api;


use actix_web::{
    server as actix_server, App, HttpRequest, HttpResponse, Result as ActixResult,
    error,
    dev,
    State,
    Json,
};
use actix::System;
use serde::{Serialize, Deserialize};
use crate::{
    server::middlewares::logger::GetRequestLogger,
    server::middlewares,
    log::macros::{slog_info, slog_o}
};


#[derive(Clone, Debug, Deserialize, Serialize)]
struct HelloWorld<'a> {
    pub hello: &'a str,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewUserRequest {
    pub name: String,
}

fn index(req: &HttpRequest<api::State>) -> ActixResult<HttpResponse> {
    let res = HelloWorld{hello: "world"};
    let logger = req.logger();
    slog_info!(logger, "hello world");
    return Ok(HttpResponse::Ok().json(api::Response::data(res)));
}


fn json_default_config(cfg: &mut (dev::JsonConfig<api::State>, ())) {
    cfg.0.error_handler(|err, _req| {  // <- create custom error response
        api::Error::BadClientData{error: err.to_string()}.into()
    });
}

fn main() {
    let (_guard, logger) = log::setup_slog();
    let sys = System::new("kernel");

    actix_server::new(|| {
        App::with_state(api::State{})
        .middleware(middlewares::request_id::RequestIDHeader)
        .middleware(middlewares::logger::Logger)
        .resource("/", |r| r.method(http::Method::GET).f(index))
        .resource("/hello", |r| r.method(http::Method::GET).f(index))
        .default_resource(|r| r.f(api::route_404))
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

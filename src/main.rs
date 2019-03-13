#[macro_use]
extern crate failure;
#[macro_use]
extern crate diesel;

mod db;
mod log;
mod api;
mod services;
mod error;


use actix_web::{
    server as actix_server, App, HttpRequest, HttpResponse, Result as ActixResult,
    middleware::cors::Cors,
    http::{header},
};
use actix::System;
use serde::{Serialize, Deserialize};
use crate::{
    api::middlewares::logger::GetRequestLogger,
    api::middlewares,
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

fn main() {
    let (_guard, logger) = log::setup_slog();
    let sys = System::new("kernel");
    let db_actor_addr = db::init();
    let api_state = api::State{
        db: db_actor_addr,
    };

    actix_server::new(move || {
        App::with_state(api_state.clone())
        .middleware(middlewares::request_id::RequestIDHeader)
        .middleware(middlewares::logger::Logger)
        .middleware(middlewares::DefaultHeaders)
        .default_resource(|r| r.f(api::route_404))
        .configure(|app| {
            Cors::for_app(app)
                // .allowed_origin("*")
                .send_wildcard()
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![header::ORIGIN, header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
                .max_age(3600)
                .resource("/", |r| r.method(http::Method::GET).f(index))
                .resource("/account/v1/register", |r| r.method(http::Method::POST).with_config(services::account::api::v1::post_register, api::json_default_config))
                .resource("/hello", |r| r.method(http::Method::GET).f(index))
                .register()
        })
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

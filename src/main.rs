#[macro_use]
extern crate failure;

mod db;
mod server;
mod log;


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

pub struct AppState {}

#[derive(Fail, Debug)]
enum MyError {
    #[fail(display="Internal error")]
    Internal,
    #[fail(display="Bad request: {}", error)]
    BadClientData{ error: String },
    #[fail(display="Timeout")]
    Timeout,
    #[fail(display="Route not found")]
    RouteNotFound,
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        let res: Response<String> = Response::error(&format!("{}", self));
       match *self {
          MyError::Internal => HttpResponse::InternalServerError().json(&res),
          MyError::BadClientData{..} => HttpResponse::BadRequest().json(&res),
          MyError::Timeout => HttpResponse::new(http::StatusCode::GATEWAY_TIMEOUT),
          MyError::RouteNotFound => HttpResponse::NotFound().json(&res),
       }
    }
}

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewUserRequest {
    pub name: String,
}

fn post_users(new_user: Json<NewUserRequest>, state: State<AppState>) -> ActixResult<HttpResponse> {
    return Ok(HttpResponse::Ok().json(Response::data(new_user.0)));
}

fn index(req: &HttpRequest<AppState>) -> ActixResult<HttpResponse> {
    let res = HelloWorld{hello: "world"};
    let logger = req.logger();
    slog_info!(logger, "hello world");
    return Ok(HttpResponse::Ok().json(Response::data(res)));
}

fn route_404(_req: &HttpRequest<AppState>) -> ActixResult<HttpResponse> {
    return Err(MyError::RouteNotFound.into());
}

fn json_default_config(cfg: &mut (dev::JsonConfig<AppState>, ())) {
    cfg.0.error_handler(|err, _req| {  // <- create custom error response
        MyError::BadClientData{error: err.to_string()}.into()
    });
}

fn main() {
    let (_guard, logger) = log::setup_slog();
    let sys = System::new("kernel");

    actix_server::new(|| {
        App::with_state(AppState{})
        .middleware(middlewares::request_id::RequestIDHeader)
        .middleware(middlewares::logger::Logger)
        .resource("/", |r| r.method(http::Method::GET).f(index))
        .resource("/users", |r| r.method(http::Method::POST).with_config(post_users, json_default_config))
        .resource("/hello", |r| r.method(http::Method::GET).f(index))
        .default_resource(|r| r.f(route_404))
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

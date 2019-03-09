mod db;
mod server;

use actix_web::{server as actix_server, App, HttpRequest, HttpResponse, Result as ActixResult};
use actix::System;
use serde::{Serialize, Deserialize};
use server::middlewares;

#[derive(Serialize)]
pub struct Response<T: Serialize> {
    data: Option<T>,
    error: Option<String>
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
            error: Some(err.to_string()),
        };
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct HelloWorld<'a> {
    pub hello: &'a str,
}

fn index(_req: &HttpRequest) -> ActixResult<HttpResponse> {
    let res = HelloWorld{hello: "world"};
    return Ok(HttpResponse::Ok().json(Response::data(res)));
}

fn main() {
    let sys = System::new("kernel");

    actix_server::new(|| {
        App::new()
        .middleware(middlewares::request_id::RequestIDHeader)
        .resource("/", |r| r.method(http::Method::GET).f(index))
    })
    .backlog(8192)
    .bind("127.0.0.1:8000")
    .unwrap()
    .keep_alive(actix_server::KeepAlive::Timeout(60))
    .shutdown_timeout(2)
    .workers(8)
    .start();

    println!("Started kernel: 127.0.0.1:8000");
    let _ = sys.run();
}

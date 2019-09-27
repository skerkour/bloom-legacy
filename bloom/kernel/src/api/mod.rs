mod state;

use crate::{messages, KernelError};
use actix_web::{web::JsonConfig, HttpResponse, ResponseError, Result as ActixResult};
use serde::{Deserialize, Serialize};

pub mod middlewares;
pub use state::State;

#[derive(Serialize)]
pub struct Response<T: Serialize> {
    pub data: Option<T>,
    pub error: Option<BlmResponseError>,
}

#[derive(Serialize)]
pub struct BlmResponseError {
    message: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct HelloWorld<'a> {
    pub hello: &'a str,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NoData {}

impl<T: Serialize> Response<T> {
    pub fn data(data: T) -> Response<T> {
        return Response {
            data: Some(data),
            error: None,
        };
    }

    pub fn error(err: KernelError) -> Response<T> {
        return Response {
            data: None,
            error: Some(BlmResponseError {
                message: format!("{}", err),
            }),
        };
    }
}

pub fn response<D: Into<messages::Message>>(data: D) -> HttpResponse {
    let message: messages::Message = data.into();
    HttpResponse::Ok().json(&message)
}

pub fn index() -> ActixResult<HttpResponse> {
    let res = response(messages::kernel::HelloWorld {
        hello: "world".to_string(),
    });
    return Ok(res);
}

pub fn route_404() -> ActixResult<HttpResponse> {
    return Ok(KernelError::RouteNotFound.error_response());
}

pub fn route_disabled() -> ActixResult<HttpResponse> {
    return Ok(KernelError::Validation("Service is disabled".to_string()).error_response());
}

pub fn json_default_config() -> JsonConfig {
    return JsonConfig::default()
        .limit(256_000)
        .error_handler(|err, _| {
            KernelError::Validation(err.to_string()).into() // <- create custom error response
        });
}

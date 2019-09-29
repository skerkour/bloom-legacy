mod state;

use crate::KernelError;
use actix_web::{web::JsonConfig, HttpResponse, Result as ActixResult};
// use serde::{Deserialize, Serialize};

pub mod middlewares;
pub use state::State;

pub fn response<D: Into<messages::Message>>(data: D) -> HttpResponse {
    let message: messages::Message = data.into();
    HttpResponse::Ok().json(&message)
}

pub fn route_404() -> ActixResult<HttpResponse> {
    let err: messages::kernel::Error = KernelError::RouteNotFound.into();
    return Ok(response(err));
}

pub fn route_disabled() -> ActixResult<HttpResponse> {
    let err: messages::kernel::Error =
        KernelError::Validation("Service is disabled".to_string()).into();
    return Ok(response(err));
}

pub fn json_default_config() -> JsonConfig {
    return JsonConfig::default()
        .limit(256_000)
        .error_handler(|err, _| {
            KernelError::Validation(err.to_string()).into() // <- create custom error response
        });
}

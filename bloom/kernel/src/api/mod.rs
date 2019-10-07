mod state;

use actix_web::{web::JsonConfig, HttpResponse, Result as ActixResult};
use bloom_error::BloomError;
// use serde::{Deserialize, Serialize};

pub mod middlewares;
pub use state::State;

pub fn response<D: Into<bloom_messages::Message>>(data: D) -> HttpResponse {
    let message: bloom_messages::Message = data.into();
    HttpResponse::Ok().json(&message)
}

pub fn route_404() -> ActixResult<HttpResponse> {
    let err: bloom_messages::kernel::Error = BloomError::RouteNotFound.into();
    return Ok(response(err));
}

pub fn route_disabled() -> ActixResult<HttpResponse> {
    let err: bloom_messages::kernel::Error =
        BloomError::Validation("Service is disabled".to_string()).into();
    return Ok(response(err));
}

pub fn json_default_config() -> JsonConfig {
    return JsonConfig::default()
        .limit(256_000)
        .error_handler(|err, _| {
            BloomError::Validation(err.to_string()).into() // <- create custom error response
        });
}

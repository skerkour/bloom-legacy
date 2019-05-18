mod state;

use serde::{Deserialize, Serialize};
use actix_web::{
    Result as ActixResult,
    HttpResponse,
    HttpRequest,
    web::JsonConfig,
};
use crate::KernelError;


pub mod middlewares;
pub use state::State;


#[derive(Serialize)]
pub struct Response<T: Serialize> {
    pub data: Option<T>,
    pub error: Option<ResponseError>
}

#[derive(Serialize)]
pub struct ResponseError{
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
        return Response{
            data: Some(data),
            error: None,
        };
    }

    pub fn error(err: KernelError) -> Response<T> {
        return Response{
            data: None,
            error: Some(ResponseError{message: format!("{}", err)}),
        };
    }
}

pub fn index() -> ActixResult<HttpResponse> {
    let res = HelloWorld{hello: "world"};
    return Ok(HttpResponse::Ok().json(Response::data(res)));
}

pub fn route_404() -> ActixResult<HttpResponse> {
    let err = KernelError::RouteNotFound;
    let res: Response<()> = Response::error(err);
    return Ok(HttpResponse::NotFound().json(res));
}

pub fn json_default_config() -> JsonConfig {
    return JsonConfig::default()
        .limit(256000)
        .error_handler(|err, _| {
            KernelError::Validation(err.to_string()).into() // <- create custom error response
        });
}

// pub fn json_default_config(cfg: &mut ((dev::JsonConfig<State>, ()),)) {
//     (cfg.0).0.error_handler(|err, _req| {  // <- create custom error response
//         KernelError::Validation(err.to_string()).into()
//     });
// }

// pub fn json_default_config_path(cfg: &mut ((PathConfig<State>, dev::JsonConfig<State>, ()),)) {
//     (cfg.0).1.error_handler(|err, _req| {  // <- create custom error response
//         KernelError::Validation(err.to_string()).into()
//     });
// }

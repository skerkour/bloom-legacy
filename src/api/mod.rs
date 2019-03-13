mod error;
mod state;

use serde::Serialize;
use actix_web::{
    Result as ActixResult,
    HttpResponse,
    HttpRequest,
    dev,
};


pub mod middlewares;
pub use error::Error;
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

impl<T: Serialize> Response<T> {
    pub fn data(data: T) -> Response<T> {
        return Response{
            data: Some(data),
            error: None,
        };
    }

    pub fn error(err: error::Error) -> Response<T> {
        return Response{
            data: None,
            error: Some(ResponseError{message: format!("{}", err)}),
        };
    }
}


pub fn route_404(_req: &HttpRequest<State>) -> ActixResult<HttpResponse> {
    return Err(Error::RouteNotFound.into());
}

pub fn json_default_config(cfg: &mut (dev::JsonConfig<State>, ())) {
    cfg.0.error_handler(|err, _req| {  // <- create custom error response
        Error::BadRequest{error: err.to_string()}.into()
    });
}

use failure::Fail;
use actix_web::{
    error,
    HttpResponse,
};
use crate::api::Response;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display="Internal error")]
    Internal,
    #[fail(display="Bad request: {}", error)]
    BadRequest{ error: String },
    #[fail(display="Timeout")]
    Timeout,
    #[fail(display="Route not found")]
    RouteNotFound,
}

impl error::ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        let res: Response<String> = Response::error(&format!("{}", self));
        match *self {
            Error::Internal => HttpResponse::InternalServerError().json(&res),
            Error::BadRequest{..} => HttpResponse::BadRequest().json(&res),
            Error::Timeout => HttpResponse::RequestTimeout().json(&res),
            Error::RouteNotFound => HttpResponse::NotFound().json(&res),
        }
    }
}

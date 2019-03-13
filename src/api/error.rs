use failure::Fail;
use actix_web::{
    error,
    HttpResponse,
};
use crate::{
    error::KernelError,
    api::Response,
};

#[derive(Clone, Debug, Fail)]
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
        let res: Response<String> = Response::error(self.clone());
        match *self {
            Error::Internal => HttpResponse::InternalServerError().json(&res),
            Error::BadRequest{..} => HttpResponse::BadRequest().json(&res),
            Error::Timeout => HttpResponse::RequestTimeout().json(&res),
            Error::RouteNotFound => HttpResponse::NotFound().json(&res),
        }
    }
}

impl std::convert::From<KernelError> for Error {
    fn from(e: KernelError) -> Self {
        Error::Internal
    }
}

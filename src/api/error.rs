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
    // 400
    #[fail(display="{}", error)]
    BadRequest{ error: String },

    // 404
    #[fail(display="Route not found")]
    RouteNotFound,

    // 408
    #[fail(display="Timeout")]
    Timeout,

    // 500
    #[fail(display="Internal error")]
    Internal,
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
        match e {
            KernelError::Validation(m) => Error::BadRequest{error: m},
            _ => Error::Internal,
        }
        // Error::Internal
    }
}

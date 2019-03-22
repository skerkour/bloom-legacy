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

    // 401
    #[fail(display="{}", 0)]
    Unauthorized(String),

    // 403
    #[fail(display = "{}", 0)]
    Forbidden(String),

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
        let res: Response<()> = Response::error(self.clone());
        match *self {
            Error::BadRequest{..} => HttpResponse::BadRequest().json(&res),
            Error::Unauthorized(_) => HttpResponse::Unauthorized().json(&res),
            Error::Forbidden(_) => HttpResponse::Forbidden().json(&res),
            Error::RouteNotFound => HttpResponse::NotFound().json(&res),
            Error::Timeout => HttpResponse::RequestTimeout().json(&res),
            Error::Internal => HttpResponse::InternalServerError().json(&res),
        }
    }
}

impl std::convert::From<KernelError> for Error {
    fn from(e: KernelError) -> Self {
        match e {
            KernelError::Validation(m) => Error::BadRequest{error: m},
            KernelError::Unauthorized(m) => Error::Unauthorized(m),
            _ => Error::Internal,
        }
    }
}

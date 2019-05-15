use failure::Fail;
use actix_web::{
    error,
    HttpResponse,
};
use std::io;
use crate::{
    api::Response,
};

#[derive(Clone, Debug, Fail)]
pub enum KernelError {
    #[fail(display="ActixMailboxError")]
    ActixMailbox,

    #[fail(display="TokioTimerError: {}", 0)]
    TokioTimer(String),

    #[fail(display="DieselError: {}", 0)]
    Diesel(String),

    #[fail(display="R2d2Error")]
    R2d2,

    #[fail(display="BcryptError")]
    Bcrypt,

    #[fail(display = "Not valid: {}", 0)]
    Validation(String),

    #[fail(display = "Unauthorized: {}", 0)]
    Unauthorized(String),

    #[fail(display = "Io: {}", 0)]
    Io(String),

    #[fail(display = "Image: {}", 0)]
    Image(String),

    #[fail(display = "NotFound: {}", 0)]
    NotFound(String),

    #[fail(display="Route not found")]
    RouteNotFound,

    #[fail(display="Timeout")]
    Timeout,

    #[fail(display="Internal error")]
    Internal(String),

    #[fail(display="URL parse error: {}", 0)]
    UrlParseError(url::ParseError),

    #[fail(display="Zip: {:?}", 0)]
    Zip(String),

    #[fail(display="Walkdir: {:?}", 0)]
    Walkdir(String),
}


impl std::convert::From<actix::MailboxError> for KernelError {
    fn from(_e: actix::MailboxError) -> Self {
        KernelError::ActixMailbox
    }
}

impl std::convert::From<tokio_timer::Error> for KernelError {
    fn from(err: tokio_timer::Error) -> Self {
        KernelError::TokioTimer(format!("{:?}", err))
    }
}

impl std::convert::From<diesel::result::Error> for KernelError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            e @ diesel::result::Error::NotFound => KernelError::NotFound(format!("{}", e)),
            e @ _ => KernelError::Diesel(format!("{:?}", e)),
        }
    }
}

impl std::convert::From<image::ImageError> for KernelError {
    fn from(err: image::ImageError) -> Self {
        KernelError::Image(format!("{:?}", err))
    }
}


impl std::convert::From<diesel::r2d2::Error> for KernelError {
    fn from(_err: diesel::r2d2::Error) -> Self {
        KernelError::R2d2
    }
}

impl std::convert::From<bcrypt::BcryptError> for KernelError {
    fn from(_err: bcrypt::BcryptError) -> Self {
        KernelError::Bcrypt
    }
}

impl std::convert::From<url::ParseError> for KernelError {
    fn from(err: url::ParseError) -> Self {
        KernelError::UrlParseError(err)
    }
}

impl std::convert::From<io::Error> for KernelError {
    fn from(err: io::Error) -> Self {
        KernelError::Io(format!("{:?}", err))
    }
}

impl std::convert::From<zip::result::ZipError> for KernelError {
    fn from(err: zip::result::ZipError) -> Self {
        KernelError::Zip(format!("{:?}", err))
    }
}

impl std::convert::From<walkdir::Error> for KernelError {
    fn from(err: walkdir::Error) -> Self {
        KernelError::Walkdir(format!("{:?}", err))
    }
}


impl error::ResponseError for KernelError {
    fn error_response(&self) -> HttpResponse {
        let res: Response<()> = Response::error(self.clone());
        match *self {
            // 400
            KernelError::Validation(_) | KernelError::UrlParseError(_) => HttpResponse::BadRequest().json(&res),
            // 401
            KernelError::Unauthorized(_) => HttpResponse::Unauthorized().json(&res),
            // 404
            KernelError::NotFound(_) | KernelError::RouteNotFound => HttpResponse::NotFound().json(&res),
            // 408
            KernelError::Timeout => HttpResponse::RequestTimeout().json(&res),
            // 500
            // KernelError::ActixMailbox | KernelError::Diesel(_) | KernelError::R2d2 | KernelError::TokioTimer(_)
            // | KernelError::Bcrypt | KernelError::Io(_) | KernelError::Image(_) | KernelError::Internal(_)
            // | KernelError::Zip(_) | KernelError::Walkdir(_) => {
            _ => {
                let res: Response<()> = Response::error(KernelError::Internal(String::new()));
                HttpResponse::InternalServerError().json(&res)
            },
        }
    }
}

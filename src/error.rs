use failure::Fail;
use actix_web::{
    error,
    HttpResponse,
};
use crate::{
    api::Response,
};

#[derive(Clone, Debug, Fail)]
pub enum KernelError {
    #[fail(display="ActixMailboxError")]
    ActixMailbox,

    #[fail(display="DieselError: {}", 0)]
    Diesel(String),

    #[fail(display="R2d2Error")]
    R2d2,

    #[fail(display="TokioError")]
    Tokio,

    #[fail(display="BcryptError")]
    Bcrypt,

    #[fail(display = "validation error: {}", 0)]
    Validation(String),

    #[fail(display = "Unauthorized: {}", 0)]
    Unauthorized(String),

    #[fail(display = "Io: {}", 0)]
    Io(String),

    #[fail(display = "Io: {}", 0)]
    Image(String),

    #[fail(display = "NotFound: {}", 0)]
    NotFound(String),

    #[fail(display="Route not found")]
    RouteNotFound,

    #[fail(display="Timeout")]
    Timeout,
}


impl std::convert::From<actix::MailboxError> for KernelError {
    fn from(_e: actix::MailboxError) -> Self {
        KernelError::ActixMailbox
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

impl std::convert::From<tokio_timer::Error> for KernelError {
    fn from(_err: tokio_timer::Error) -> Self {
        KernelError::Tokio
    }
}

impl std::convert::From<bcrypt::BcryptError> for KernelError {
    fn from(_err: bcrypt::BcryptError) -> Self {
        KernelError::Bcrypt
    }
}


// #[derive(Clone, Debug, Fail)]
// pub enum Error {
//     // 400
//     #[fail(display="{}", error)]
//     BadRequest{ error: String },

//     // 401
//     #[fail(display="{}", 0)]
//     Unauthorized(String),

//     // 403
//     #[fail(display = "{}", 0)]
//     Forbidden(String),

//     // 404
//     #[fail(display="Route not found")]
//     RouteNotFound,

//     // 408
//     #[fail(display="Timeout")]
//     Timeout,

//     // 500
//     #[fail(display="Internal error")]
//     Internal,
// }

impl error::ResponseError for KernelError {
    fn error_response(&self) -> HttpResponse {
        let res: Response<()> = Response::error(self.clone());
        match *self {
            // 400
            KernelError::Validation(_) => HttpResponse::BadRequest().json(&res),
            // 401
            KernelError::Unauthorized(_) => HttpResponse::Unauthorized().json(&res),
            // 404
            KernelError::NotFound(_) | KernelError::RouteNotFound => HttpResponse::NotFound().json(&res),
            // 408
            KernelError::Timeout => HttpResponse::RequestTimeout().json(&res),
            // 500
            KernelError::ActixMailbox | KernelError::Diesel(_) | KernelError::R2d2 | KernelError::Tokio
            | KernelError::Bcrypt | KernelError::Io(_) | KernelError::Image(_) => HttpResponse::InternalServerError().json(&res),
        }
    }
}

use crate::api;
use actix_web::{error, HttpResponse};
use failure::Fail;
use std::io;

#[derive(Clone, Debug, Fail)]
pub enum KernelError {
    #[fail(display = "ActixMailboxError")]
    ActixMailbox,

    #[fail(display = "TokioTimerError: {}", 0)]
    TokioTimer(String),

    #[fail(display = "DieselError: {}", 0)]
    Diesel(String),

    #[fail(display = "R2d2Error")]
    R2d2,

    #[fail(display = "BcryptError")]
    Bcrypt,

    #[fail(display = "Crypto42: {}", 0)]
    Crypto42(String),

    #[fail(display = "Not valid: {}", 0)]
    Validation(String),

    #[fail(display = "Unauthorized: {}", 0)]
    Unauthorized(String),

    #[fail(display = "Forbidden: {}", 0)]
    Forbidden(String),

    #[fail(display = "Io: {}", 0)]
    Io(String),

    #[fail(display = "Image: {}", 0)]
    Image(String),

    #[fail(display = "NotFound: {}", 0)]
    NotFound(String),

    #[fail(display = "Route not found")]
    RouteNotFound,

    #[fail(display = "Timeout")]
    Timeout,

    #[fail(display = "Internal error")]
    Internal(String),

    #[fail(display = "URL parse error: {}", 0)]
    UrlParseError(url::ParseError),

    #[fail(display = "Zip: {:?}", 0)]
    Zip(String),

    #[fail(display = "Walkdir: {:?}", 0)]
    Walkdir(String),

    #[fail(display = "SerdeJson: {:?}", 0)]
    SerdeJson(String),

    #[fail(display = "LettreEmail: {:?}", 0)]
    LettreEmail(String),

    #[fail(display = "LettreSmtp: {:?}", 0)]
    LettreSmtp(String),
}

impl From<KernelError> for messages::kernel::Error {
    fn from(err: KernelError) -> Self {
        let code = match &err {
            // 400
            KernelError::Validation(_)
            | KernelError::UrlParseError(_)
            | KernelError::LettreEmail(_)
            | KernelError::LettreSmtp(_) => "VALIDATION".to_string(),
            // 401
            KernelError::Unauthorized(_) => "UNAUTHORIZED".to_string(),
            // 403
            KernelError::Forbidden(_) => "FORBIDDEN".to_string(),
            // 404
            KernelError::RouteNotFound => "ROUTE_NOT_FOUND".to_string(),
            KernelError::NotFound(_) => "NOT_FOUND".to_string(),
            // 408
            KernelError::Timeout => "TIMEOUT".to_string(),
            // 500
            _ => "INTERNAL".to_string(),
        };
        messages::kernel::Error {
            code,
            message: format!("{}", err),
        }
    }
}

impl From<actix::MailboxError> for KernelError {
    fn from(_e: actix::MailboxError) -> Self {
        KernelError::ActixMailbox
    }
}

impl From<diesel::result::Error> for KernelError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            e @ diesel::result::Error::NotFound => KernelError::NotFound(format!("{}", e)),
            e => KernelError::Diesel(format!("{:?}", e)),
        }
    }
}

impl From<image::ImageError> for KernelError {
    fn from(err: image::ImageError) -> Self {
        KernelError::Image(format!("{:?}", err))
    }
}

impl From<diesel::r2d2::Error> for KernelError {
    fn from(_err: diesel::r2d2::Error) -> Self {
        KernelError::R2d2
    }
}

impl From<url::ParseError> for KernelError {
    fn from(err: url::ParseError) -> Self {
        KernelError::UrlParseError(err)
    }
}

impl From<io::Error> for KernelError {
    fn from(err: io::Error) -> Self {
        KernelError::Io(format!("{:?}", err))
    }
}

impl From<zip::result::ZipError> for KernelError {
    fn from(err: zip::result::ZipError) -> Self {
        KernelError::Zip(format!("{:?}", err))
    }
}

impl From<walkdir::Error> for KernelError {
    fn from(err: walkdir::Error) -> Self {
        KernelError::Walkdir(format!("{:?}", err))
    }
}

impl From<serde_json::Error> for KernelError {
    fn from(err: serde_json::Error) -> Self {
        KernelError::SerdeJson(format!("{:?}", err))
    }
}

impl From<lettre_email::error::Error> for KernelError {
    fn from(err: lettre_email::error::Error) -> Self {
        KernelError::LettreEmail(format!("{:?}", err))
    }
}

impl From<lettre::smtp::error::Error> for KernelError {
    fn from(err: lettre::smtp::error::Error) -> Self {
        KernelError::LettreSmtp(format!("{:?}", err))
    }
}

impl From<crypto42::Error> for KernelError {
    fn from(err: crypto42::Error) -> Self {
        KernelError::Crypto42(format!("{:?}", err))
    }
}

impl error::ResponseError for KernelError {
    fn error_response(&self) -> HttpResponse {
        let res: messages::kernel::Error = self.clone().into();
        api::response(res)
        // let res: Response<()> = Response::error(self.clone());
        // match *self {
        //     // 400
        //     KernelError::Validation(_)
        //     | KernelError::UrlParseError(_)
        //     | KernelError::LettreEmail(_)
        //     | KernelError::LettreSmtp(_) => HttpResponse::BadRequest().json(&res),
        //     // 401
        //     KernelError::Unauthorized(_) => HttpResponse::Unauthorized().json(&res),
        //     // 403
        //     KernelError::Forbidden(_) => HttpResponse::Forbidden().json(&res),
        //     // 404
        //     KernelError::NotFound(_) | KernelError::RouteNotFound => {
        //         HttpResponse::NotFound().json(&res)
        //     }
        //     // 408
        //     KernelError::Timeout => HttpResponse::RequestTimeout().json(&res),
        //     // 500
        //     // KernelError::ActixMailbox | KernelError::Diesel(_) | KernelError::R2d2 | KernelError::TokioTimer(_)
        //     // | KernelError::Bcrypt | KernelError::Io(_) | KernelError::Image(_) | KernelError::Internal(_)
        //     // | KernelError::Zip(_) | KernelError::Walkdir(_) => {
        //     _ => {
        //         let res: messages::kernel::Error = KernelError::Internal(String::new()).into();
        //         HttpResponse::InternalServerError().json(&api::responseres)
        //     }
        // }
    }
}

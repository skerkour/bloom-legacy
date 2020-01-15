use failure::Fail;
use std::io;

mod code;

pub use code::ErrorCode;

#[derive(Debug, Fail)]
pub enum BloomError {
    #[cfg(feature = "reqwest")]
    #[fail(display = "Reqwest: {}", 0)]
    Reqwest(reqwest::Error),

    #[cfg(feature = "crypto42")]
    #[fail(display = "Crypto42: {:?}", 0)]
    Crypto42(crypto42::Error),

    #[fail(display = "Unknown message type")]
    UnknownMessageType,

    #[fail(display = "ActixMailboxError")]
    ActixMailbox,

    #[fail(display = "TokioTimerError: {}", 0)]
    TokioTimer(String),

    #[fail(display = "DieselError: {}", 0)]
    Diesel(String),

    #[fail(display = "R2d2Error: {}", 0)]
    R2d2(String),

    #[fail(display = "Not valid: {}", 0)]
    Validation(String),

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
    UrlParseError(String),

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

    #[cfg(feature = "rusqlite")]
    #[fail(display = "Rusqlite: {:?}", 0)]
    Rusqlite(rusqlite::Error),

    #[cfg(feature = "caldyn")]
    #[fail(display = "Caldyn: {:?}", 0)]
    Caldyn(caldyn::Error),
}

// see https://github.com/turtl/core-rs/blob/9b2d93285ce07f1bb6de829a0dfc99a0bb382f71/clouseau/src/lib.rs
// /// A macro to make it easy to create From impls for CError
// macro_rules! from_err {
//     ($t:ty) => (
//         impl From<$t> for BloomError {
//             fn from(err: $t) -> CError {
//                 CError::Boxed(Box::new(err))
//             }
//         }
//     )
// }

// TODO: improve...
// TODO(z0mbie42):
// * into message
impl From<&BloomError> for bloom_messages::kernel::Error {
    fn from(err: &BloomError) -> Self {
        match err {
            BloomError::UnknownMessageType => bloom_messages::kernel::Error {
                code: ErrorCode::UKNOWN_MESSAGE_TYPE.to_string(),
                message: "Unknown message type".to_string(),
            },
            BloomError::Forbidden(message) => bloom_messages::kernel::Error {
                code: ErrorCode::FORBIDDEN.to_string(),
                message: message.clone(),
            },
            _ => bloom_messages::kernel::Error {
                code: ErrorCode::INTERNAL.to_string(),
                message: "Internal error".to_string(),
            },
        }
    }
}

impl From<BloomError> for bloom_messages::kernel::Error {
    fn from(err: BloomError) -> Self {
        return (&err).into();
    }
}

#[cfg(feature = "reqwest")]
impl From<reqwest::Error> for BloomError {
    fn from(err: reqwest::Error) -> Self {
        BloomError::Reqwest(err)
    }
}

#[cfg(feature = "crypto42")]
impl From<crypto42::Error> for BloomError {
    fn from(err: crypto42::Error) -> Self {
        BloomError::Crypto42(err)
    }
}

#[cfg(feature = "actix")]
impl From<actix::MailboxError> for BloomError {
    fn from(_e: actix::MailboxError) -> Self {
        BloomError::ActixMailbox
    }
}

#[cfg(feature = "image")]
impl From<image::ImageError> for BloomError {
    fn from(err: image::ImageError) -> Self {
        BloomError::Image(format!("{:?}", err))
    }
}

#[cfg(feature = "r2d2")]
impl From<r2d2::Error> for BloomError {
    fn from(err: r2d2::Error) -> Self {
        BloomError::R2d2(format!("{:?}", err))
    }
}

#[cfg(feature = "diesel")]
impl From<diesel::result::Error> for BloomError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            e @ diesel::result::Error::NotFound => BloomError::NotFound(format!("{}", e)),
            e => BloomError::Diesel(format!("{:?}", e)),
        }
    }
}

#[cfg(feature = "url")]
impl From<url::ParseError> for BloomError {
    fn from(err: url::ParseError) -> Self {
        BloomError::UrlParseError(format!("{}", err))
    }
}

impl From<io::Error> for BloomError {
    fn from(err: io::Error) -> Self {
        BloomError::Io(format!("{:?}", err))
    }
}

#[cfg(feature = "zip")]
impl From<zip::result::ZipError> for BloomError {
    fn from(err: zip::result::ZipError) -> Self {
        BloomError::Zip(format!("{:?}", err))
    }
}

#[cfg(feature = "walkdir")]
impl From<walkdir::Error> for BloomError {
    fn from(err: walkdir::Error) -> Self {
        BloomError::Walkdir(format!("{:?}", err))
    }
}

#[cfg(feature = "serde_json")]
impl From<serde_json::Error> for BloomError {
    fn from(err: serde_json::Error) -> Self {
        BloomError::SerdeJson(format!("{:?}", err))
    }
}

#[cfg(feature = "lettre_email")]
impl From<lettre_email::error::Error> for BloomError {
    fn from(err: lettre_email::error::Error) -> Self {
        BloomError::LettreEmail(format!("{:?}", err))
    }
}

#[cfg(feature = "lettre")]
impl From<lettre::smtp::error::Error> for BloomError {
    fn from(err: lettre::smtp::error::Error) -> Self {
        BloomError::LettreSmtp(format!("{:?}", err))
    }
}

#[cfg(feature = "rusqlite")]
impl From<rusqlite::Error> for BloomError {
    fn from(err: rusqlite::Error) -> Self {
        BloomError::Rusqlite(err)
    }
}

#[cfg(feature = "caldyn")]
impl From<caldyn::Error> for BloomError {
    fn from(err: caldyn::Error) -> Self {
        BloomError::Caldyn(err)
    }
}

#[cfg(feature = "actix-http")]
#[allow(clippy::clone_double_ref)]
impl actix_http::error::ResponseError for BloomError {
    fn error_response(&self) -> actix_http::Response {
        let res: bloom_messages::kernel::Error = self.clone().into();
        let message: bloom_messages::Message = res.into();
        actix_http::Response::Ok().json(&message)
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
        //         let res: bloom_messages::kernel::Error = KernelError::Internal(String::new()).into();
        //         HttpResponse::InternalServerError().json(&api::responseres)
        //     }
        // }
    }
}

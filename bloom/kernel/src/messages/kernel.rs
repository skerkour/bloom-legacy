use crate::error::KernelError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NoData {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HelloWorld {
    pub hello: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Error {
    pub code: String,
    pub message: String,
}

impl From<KernelError> for Error {
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
            KernelError::NotFound(_) => "ROUTE_NOT_FOUND".to_string(),
            // 408
            KernelError::Timeout => "TIMEOUT".to_string(),
            // 500
            _ => "INTERNAL".to_string(),
        };
        Error {
            code,
            message: format!("{}", err),
        }
    }
}

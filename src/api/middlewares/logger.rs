use actix_web::{
    Result, Error, HttpResponse, HttpRequest, FromRequest,
    middleware::{Started, Finished, Middleware},
    http::header,
};
use chrono::{Utc, DateTime};
use crate::api::middlewares::{
    GetRequestId,
    RequestId,
};
use slog::{slog_o, slog_info, slog_warn, slog_error};
use std::ops::Deref;


#[derive(Clone, Debug)]
pub struct RequestLogger(slog::Logger);

pub trait GetRequestLogger {
    /// Returns the HttpRequest RequestID, if the HttpRequest currently has none
    /// it creates one and associates it to the HttpRequest.
    fn logger(&self) -> RequestLogger;
}

impl<S> GetRequestLogger for HttpRequest<S> {
    fn logger(&self) -> RequestLogger {

        let req_id =  self.request_id().0.clone();

        // Return the current logger augmented with the request_id
        let new_log = slog_scope::logger().new(slog_o!("request_id" => req_id));

        // Ok(RequestLogger(new_log))

        // let id = uuid::Uuid::new_v4().to_string();
        self.extensions_mut().insert(RequestLogger(new_log.clone()));
        RequestLogger(new_log)
    }
}


struct RequestStartTime(DateTime<Utc>);

impl RequestLogger {
    /// Deconstruct to an inner value
    pub fn into_inner(self) -> slog::Logger {
        self.0
    }

    /// Execute code in the request's logging scope. Convenience wrapper around `slog_scope::scope()`.
    #[inline]
    pub fn scope<SF, R>(&self, f: SF) -> R
    where
        SF: FnOnce() -> R,
    {
        slog_scope::scope(&self.0, f)
    }
}

pub struct LoggerMiddleware;

impl<S> Middleware<S> for LoggerMiddleware {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        req.extensions_mut().insert(RequestStartTime(Utc::now()));
        return Ok(Started::Done);
    }

    fn finish(&self, req: &HttpRequest<S>, resp: &HttpResponse) -> Finished {
        let logger = req.logger();

        let start_time: RequestStartTime = req.extensions_mut().remove().unwrap();
        let now = Utc::now();
        let duration = now.signed_duration_since(start_time.0).num_milliseconds();

        let connection_info = req.connection_info();
        let remote = if let Some(remote) = connection_info.remote() {
            remote
        } else {
            "-"
        };
        let status = resp.status().as_u16();
        let default_user_agent = header::HeaderValue::from_str("").unwrap();
        let fields =  slog_o!(
            "duration" => duration,
            "size" => resp.response_size(),
            "status" => status,
            "scheme" => connection_info.scheme(),
            "url" => req.path(),
            "method" => req.method().as_str(),
            "remote_address" => remote,
            // TODO: not working
            "user_agent" => resp.headers().get(header::USER_AGENT).unwrap_or(&default_user_agent).to_str().unwrap(),
        );
        match status {
            x if x < 400 => { slog_info!(logger, "access"; fields) },
            x if x < 500 => { slog_warn!(logger, "access"; fields) },
            _  => { slog_error!(logger, "access"; fields) },
        }

        // slog_info!(logger, "access";

        // );
        return Finished::Done;
    }
}


impl<S> FromRequest<S> for RequestLogger {
    type Config = ();
    type Result = Result<Self, Error>;

    #[inline]
    fn from_request(req: &HttpRequest<S>, _: &Self::Config) -> Self::Result {
        // String processing because request_id.0 is private
        let req_id = req.request_id().0.clone();

        // Return the current logger augmented with the request_id
        let new_log = slog_scope::logger().new(slog_o!("request_id" => req_id));

        Ok(RequestLogger(new_log))
    }
}

/// Allow direct access to `Logger` methods from a `RequestLogger`.
impl Deref for RequestLogger {
    type Target = slog::Logger;

    fn deref(&self) -> &slog::Logger {
        &self.0
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test::TestRequest;

    #[test]
    fn request_get_logger() {
        let req = TestRequest::default().finish();
        let _ = req.logger();
    }
}

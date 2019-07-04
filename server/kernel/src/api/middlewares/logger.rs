// use actix_web::{
//     Result, Error, HttpResponse, HttpRequest, FromRequest,
//     middleware::{Started, Finished, Middleware},
//     http::header,
// };
use actix_web::{
    dev::Payload,
    // http::header::HeaderValue,
    // dev::{ServiceRequest, ServiceResponse},
    // web,
    Error,
    FromRequest,
    HttpRequest,
};
// use actix_service::{Service as ActixService, Transform};
// use chrono::{Utc, DateTime};
use slog::slog_o; // slog_info, slog_warn, slog_error};
use std::ops::Deref;
// use futures::{
//     Poll,
//     future::{ok, Either, FutureResult},
//     Future,
// };
use crate::api::middlewares::GetRequestId;

#[derive(Clone, Debug)]
pub struct RequestLogger(slog::Logger);

pub trait GetRequestLogger {
    /// Returns the HttpRequest RequestID, if the HttpRequest currently has none
    /// it creates one and associates it to the HttpRequest.
    fn logger(&self) -> RequestLogger;
}

impl GetRequestLogger for HttpRequest {
    fn logger(&self) -> RequestLogger {
        let req_id = self.request_id().0;

        // Return the current logger augmented with the request_id
        let new_log = slog_scope::logger().new(slog_o!("request_id" => req_id.to_string()));

        // Ok(RequestLogger(new_log))

        // let id = uuid::Uuid::new_v4().to_string();
        self.extensions_mut().insert(RequestLogger(new_log.clone()));
        RequestLogger(new_log)
    }
}

// struct RequestStartTime(DateTime<Utc>);

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

impl FromRequest for RequestLogger {
    type Config = ();
    type Error = Error;
    type Future = Result<RequestLogger, Error>;

    #[inline]
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        // String processing because request_id.0 is private
        let req_id = req.request_id().0;

        // Return the current logger augmented with the request_id
        let new_log = slog_scope::logger().new(slog_o!("request_id" => req_id.to_string()));

        return Ok(RequestLogger(new_log));
    }
}

/// Allow direct access to `Logger` methods from a `RequestLogger`.
impl Deref for RequestLogger {
    type Target = slog::Logger;

    fn deref(&self) -> &slog::Logger {
        &self.0
    }
}

// /// LoggerMiddleware
// #[derive(Debug, Clone, PartialEq)]
// pub struct LoggerMiddleware;

// impl<S, B> Transform<S> for LoggerMiddleware
// where
//     S: ActixService<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//     S::Future: 'static,
// {
//     type Request = ServiceRequest;
//     type Response = ServiceResponse<B>;
//     type Error = Error;
//     type InitError = ();
//     type Transform = LoggerMiddleware2<S>;
//     type Future = FutureResult<Self::Transform, Self::InitError>;

//     fn new_transform(&self, service: S) -> Self::Future {
//         ok(LoggerMiddleware2 { service })
//     }
// }

// pub struct LoggerMiddleware2<S> {
//     service: S,
// }

// impl<S, B> ActixService for LoggerMiddleware2<S>
// where
//     S: ActixService<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//     S::Future: 'static,
// {
//     type Request = ServiceRequest;
//     type Response = ServiceResponse<B>;
//     type Error = Error;
//     type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

//     fn poll_ready(&mut self) -> Poll<(), Self::Error> {
//         self.service.poll_ready()
//     }

//     fn call(&mut self, req: ServiceRequest) -> Self::Future {
//         let state: web::Data<api::State> = req.app_data().expect("error getting app_data in auth middleware");

//     fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
//         req.extensions_mut().insert(RequestStartTime(Utc::now()));
//         return Ok(Started::Done);
//     }

//     fn finish(&self, req: &HttpRequest<S>, resp: &HttpResponse) -> Finished {
//         let logger = req.logger();

//         let start_time: RequestStartTime = req.extensions_mut().remove().unwrap();
//         let now = Utc::now();
//         let duration = now.signed_duration_since(start_time.0).num_milliseconds();

//         let connection_info = req.connection_info();
//         let remote = if let Some(remote) = connection_info.remote() {
//             remote
//         } else {
//             "-"
//         };
//         let status = resp.status().as_u16();
//         let default_user_agent = header::HeaderValue::from_str("").unwrap();
//         let fields =  slog_o!(
//             "duration" => duration,
//             "size" => resp.response_size(),
//             "status" => status,
//             "scheme" => connection_info.scheme(),
//             "url" => req.path(),
//             "method" => req.method().as_str(),
//             "remote_address" => remote,
//             // TODO: not working
//             "user_agent" => resp.headers().get(header::USER_AGENT).unwrap_or(&default_user_agent).to_str().unwrap(),
//         );
//         match status {
//             x if x < 400 => { slog_info!(logger, "access"; fields) },
//             x if x < 500 => { slog_warn!(logger, "access"; fields) },
//             _  => { slog_error!(logger, "access"; fields) },
//         }

//         // slog_info!(logger, "access";

//         // );
//         return Finished::Done;

//         let auth_header = req.headers().get(header::AUTHORIZATION);
//         if auth_header.is_none() {
//             req.extensions_mut().insert(Auth{
//                 account: None,
//                 session: None,
//                 service: None,
//             });
//             return Box::new(self.service.call(req));
//         }

//         let auth_header = auth_header.unwrap().to_str()
//             .map_err(|_| KernelError::Validation("Authorization HTTP header is not valid".to_string()))?;
//         let msg = if auth_header.starts_with("Basic ") {
//             extract_authorization_header(auth_header)
//                 .map_err(|_| KernelError::Validation("Authorization HTTP header is not valid".to_string()))?
//         } else if auth_header.starts_with("Secret ") {
//             extract_secret_header(auth_header)
//                 .map_err(|_| KernelError::Validation("Authorization HTTP header is not valid".to_string()))?
//         } else {
//             return Err(KernelError::Validation("Authorization HTTP header is not valid".to_string()).into());
//         };

//         // TODO: improve...
//         // the problem is: req.extensions_mut().insert(auth);
//         // we can either clone req or use a sync middleware
//         // the question is: what is the cost of clonning req ?
//         let req = req.clone();

//         let fut = state.db.send(msg)
//             .from_err()
//             .and_then(move |res| {
//                 match res {
//                     Ok(auth) => {
//                         req.extensions_mut().insert(auth);
//                         return Box::new(self.service.call(req));
//                     },
//                     Err(e) => Err(e.into()),
//                 }
//             });

//         return Box::new(fut);

//         // return Either::A(self.service.call(req));
//     }
// }

// impl<S> Middleware<S> for LoggerMiddleware {
//     fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
//         req.extensions_mut().insert(RequestStartTime(Utc::now()));
//         return Ok(Started::Done);
//     }

//     fn finish(&self, req: &HttpRequest<S>, resp: &HttpResponse) -> Finished {
//         let logger = req.logger();

//         let start_time: RequestStartTime = req.extensions_mut().remove().unwrap();
//         let now = Utc::now();
//         let duration = now.signed_duration_since(start_time.0).num_milliseconds();

//         let connection_info = req.connection_info();
//         let remote = if let Some(remote) = connection_info.remote() {
//             remote
//         } else {
//             "-"
//         };
//         let status = resp.status().as_u16();
//         let default_user_agent = header::HeaderValue::from_str("").unwrap();
//         let fields =  slog_o!(
//             "duration" => duration,
//             "size" => resp.response_size(),
//             "status" => status,
//             "scheme" => connection_info.scheme(),
//             "url" => req.path(),
//             "method" => req.method().as_str(),
//             "remote_address" => remote,
//             // TODO: not working
//             "user_agent" => resp.headers().get(header::USER_AGENT).unwrap_or(&default_user_agent).to_str().unwrap(),
//         );
//         match status {
//             x if x < 400 => { slog_info!(logger, "access"; fields) },
//             x if x < 500 => { slog_warn!(logger, "access"; fields) },
//             _  => { slog_error!(logger, "access"; fields) },
//         }

//         // slog_info!(logger, "access";

//         // );
//         return Finished::Done;
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use actix_web::test::TestRequest;

//     #[test]
//     fn request_get_logger() {
//         let req = TestRequest::default().finish();
//         let _ = req.logger();
//     }
// }

use actix_web::{
    HttpRequest, HttpResponse, Result,
    http::header::HeaderValue,
    middleware::{Middleware, Response},
};

/// DefaultHeaders appends to response some headers (primarily for security purpose)
#[derive(Debug, Clone, PartialEq)]
pub struct DefaultHeaders;

impl<S> Middleware<S> for DefaultHeaders {
    fn response(&self, _req: &HttpRequest<S>, mut resp: HttpResponse) -> Result<Response> {
        if let Ok(v) = HeaderValue::from_str("nosniff") {
            resp.headers_mut().append("X-Content-Type-Options", v);
        }

        if let Ok(v) = HeaderValue::from_str("Deny") {
            resp.headers_mut().append("X-Frame-Options", v);
        }

        if let Ok(v) = HeaderValue::from_str("max-age=63072000; includeSubDomains; preload") {
            resp.headers_mut().append("Strict-Transport-Security", v);
        }

        Ok(Response::Done(resp))
    }
}

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
        let headers = resp.headers_mut();
        headers.append("X-Content-Type-Options", HeaderValue::from_str("nosniff").expect("parsing default header value"));
        headers.append("X-Frame-Options", HeaderValue::from_str("Deny").expect("parsing default header value"));
        headers.append("Strict-Transport-Security", HeaderValue::from_str("max-age=63072000; includeSubDomains; preload").expect("parsing default header value"));

        return Ok(Response::Done(resp));
    }
}

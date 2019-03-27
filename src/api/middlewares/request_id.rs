use actix_web::{
    FromRequest, HttpRequest, HttpResponse, Result,
    http::header::HeaderValue,
    middleware::{Middleware, Response},
};
use std::ops::Deref;



/// The header set by the middleware
pub const REQUEST_ID_HEADER: &str = "X-Bloom-Request-ID";

/// The HTTP Request ID
///
/// **note:** must contain as String that is valid to put in HTTP Header values
///
/// It can also be extracted from a request and Helper converter to be able to extract the RequestID easily in an handler
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RequestId(pub uuid::Uuid);

/// Permits retrieving the HttpRequest associated RequestID
pub trait GetRequestId {
    /// Returns the HttpRequest RequestID, if the HttpRequest currently has none
    /// it creates one and associates it to the HttpRequest.
    fn request_id(&self) -> RequestId;
}

impl<S> GetRequestId for HttpRequest<S> {
    fn request_id(&self) -> RequestId {
        if let Some(req_id) = self.extensions().get::<RequestId>() {
            return *req_id;
        }

        let id = uuid::Uuid::new_v4();
        self.extensions_mut().insert(RequestId(id));
        RequestId(id)
    }
}

impl<S> FromRequest<S> for RequestId {
    type Config = ();
    type Result = RequestId;

    #[inline]
    fn from_request(req: &HttpRequest<S>, _: &Self::Config) -> Self::Result {
        req.request_id()
    }
}

/// Allow direct access to `String` methods from a `RequestId`.
impl Deref for RequestId {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// The RequestIdMiddleware. It sets a `request-id` HTTP header to the HttpResponse
pub struct RequestIdMiddleware;
impl<S> Middleware<S> for RequestIdMiddleware {
    fn response(&self, req: &HttpRequest<S>, mut resp: HttpResponse) -> Result<Response> {
        if let Ok(v) = HeaderValue::from_str(&(req.request_id().to_string())) {
            resp.headers_mut().append(REQUEST_ID_HEADER, v);
        }

        Ok(Response::Done(resp))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test::TestRequest;

    trait ResponseGetterHelper {
        fn response(self) -> HttpResponse;
    }
    impl ResponseGetterHelper for Response {
        fn response(self) -> HttpResponse {
            match self {
                Response::Done(resp) => resp,
                _ => panic!(),
            }
        }
    }

    #[test]
    fn request_id_is_consistent_for_same_request() {
        let req = TestRequest::default().finish();

        assert_eq!(req.request_id(), req.request_id());
        assert_eq!(req.request_id(), RequestId::extract(&req));
    }

    #[test]
    fn request_id_is_new_between_different_requests() {
        let req1 = TestRequest::default().finish();
        let req2 = TestRequest::default().finish();

        assert!(req1.request_id() != req2.request_id());
        assert_eq!(req1.request_id(), req1.request_id());
        assert_eq!(req2.request_id(), req2.request_id());
    }

    #[test]
    fn middleware_adds_request_id_in_headers() {
        let req = TestRequest::default().finish();

        let resp: HttpResponse = HttpResponse::Ok().into();
        let resp = RequestIdMiddleware.response(&req, resp).unwrap().response();

        let req_id = req.request_id();

        assert_eq!(
            resp.headers().get(REQUEST_ID_HEADER).unwrap().as_bytes(),
            req_id.to_string().as_bytes()
        );
    }
}

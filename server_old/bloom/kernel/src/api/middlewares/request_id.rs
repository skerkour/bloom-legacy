use actix_service::{Service, Transform};
use actix_web::{
    dev::Payload,
    dev::{ServiceRequest, ServiceResponse},
    http::header::{HeaderName, HeaderValue},
    Error, FromRequest, HttpMessage, HttpRequest,
};
use futures::{
    future::{ok, Future, FutureResult},
    Poll,
};
use std::ops::Deref;

/// The header set by the middleware
pub const REQUEST_ID_HEADER: &str = "x-bloom-request-id";

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

impl GetRequestId for HttpRequest {
    fn request_id(&self) -> RequestId {
        if let Some(req_id) = self.extensions().get::<RequestId>() {
            return *req_id;
        }

        let id = uuid::Uuid::new_v4();
        self.extensions_mut().insert(RequestId(id));
        RequestId(id)
    }
}

impl GetRequestId for ServiceRequest {
    fn request_id(&self) -> RequestId {
        if let Some(req_id) = self.extensions().get::<RequestId>() {
            return *req_id;
        }

        let id = uuid::Uuid::new_v4();
        self.extensions_mut().insert(RequestId(id));
        RequestId(id)
    }
}

impl FromRequest for RequestId {
    type Config = ();
    type Error = Error;
    type Future = Result<RequestId, Error>;

    #[inline]
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        return Ok(req.request_id());
    }
}

/// Allow direct access to `String` methods from a `RequestId`.
impl Deref for RequestId {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct RequestIdMiddleware;

impl<S, B> Transform<S> for RequestIdMiddleware
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequestIdMiddleware2<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RequestIdMiddleware2 { service })
    }
}

/// The RequestIdMiddleware. It sets a `request-id` HTTP header to the HttpResponse
pub struct RequestIdMiddleware2<S> {
    service: S,
}

impl<S, B> Service for RequestIdMiddleware2<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        // We only need to hook into the `start` for this middleware.
        // TODO: handle error
        let request_id = req.request_id();
        return Box::new(self.service.call(req).map(move |mut res| {
            if let Ok(v) = HeaderValue::from_str(&(request_id.to_string())) {
                res.headers_mut()
                    .append(HeaderName::from_static(REQUEST_ID_HEADER), v);
            }
            return res;
        }));
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use actix_web::test::TestRequest;

//     trait ResponseGetterHelper {
//         fn response(self) -> HttpResponse;
//     }
//     impl ResponseGetterHelper for Response {
//         fn response(self) -> HttpResponse {
//             match self {
//                 Response::Done(resp) => resp,
//                 _ => panic!(),
//             }
//         }
//     }

//     #[test]
//     fn request_id_is_consistent_for_same_request() {
//         let req = TestRequest::default().finish();

//         assert_eq!(req.request_id(), req.request_id());
//         assert_eq!(req.request_id(), RequestId::extract(&req));
//     }

//     #[test]
//     fn request_id_is_new_between_different_requests() {
//         let req1 = TestRequest::default().finish();
//         let req2 = TestRequest::default().finish();

//         assert!(req1.request_id() != req2.request_id());
//         assert_eq!(req1.request_id(), req1.request_id());
//         assert_eq!(req2.request_id(), req2.request_id());
//     }

//     #[test]
//     fn middleware_adds_request_id_in_headers() {
//         let req = TestRequest::default().finish();

//         let resp: HttpResponse = HttpResponse::Ok().into();
//         let resp = RequestIdMiddleware.response(&req, resp).unwrap().response();

//         let req_id = req.request_id();

//         assert_eq!(
//             resp.headers().get(REQUEST_ID_HEADER).unwrap().as_bytes(),
//             req_id.to_string().as_bytes()
//         );
//     }
// }

// use actix_web::{
//     HttpRequest, Result, Error,
//     middleware::{Middleware, Started},
//     http::header,
// };
use crate::{accounts::domain, api, db::DbActor};
use actix::{Handler, Message};
use actix_service::{Service as ActixService, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    http::header,
    web, Error, HttpMessage, HttpRequest, ResponseError,
};
use bloom_error::BloomError;
use crypto42::kdf::argon2id;
use futures::Future;
use futures::{
    future::{ok, Either, FutureResult},
    Poll,
};
use std::cell::RefCell;
use std::env;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Auth {
    pub account: Option<domain::Account>,
    pub session: Option<domain::Session>,
    pub service: Option<Service>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Service {
    Phaser,
    Bitflow,
}

/// AuthMiddleware
#[derive(Debug, Clone, PartialEq)]
pub struct AuthMiddleware;

impl<S, B> Transform<S> for AuthMiddleware
where
    S: ActixService<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware2<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware2 {
            service: Rc::new(RefCell::new(service)),
        })
    }
}

/// The RequestIdMiddleware. It sets a `request-id` HTTP header to the HttpResponse
pub struct AuthMiddleware2<S> {
    service: Rc<RefCell<S>>,
}

impl<S, B> ActixService for AuthMiddleware2<S>
where
    S: ActixService<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = S::Error;
    type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let state: web::Data<api::State> = req
            .app_data()
            .expect("error getting app_data in auth middleware");

        let auth_header = req.headers().get(header::AUTHORIZATION);
        if auth_header.is_none() {
            req.extensions_mut().insert(Auth {
                account: None,
                session: None,
                service: None,
            });
            return Box::new(self.service.call(req));
        }

        let auth_header =
            try_future_box!(auth_header
                .unwrap()
                .to_str()
                .map_err(|_| BloomError::Validation(
                    "Authorization HTTP header is not valid".to_string()
                )));
        let msg = if auth_header.starts_with("Basic ") {
            try_future_box!(extract_authorization_header(auth_header).map_err(|_| {
                BloomError::Validation("Authorization HTTP header is not valid".to_string())
            }))
        } else if auth_header.starts_with("Secret ") {
            try_future_box!(extract_secret_header(auth_header).map_err(|_| {
                BloomError::Validation("Authorization HTTP header is not valid".to_string())
            }))
        } else {
            return Box::new(ok(req.error_response(
                BloomError::Validation("Authorization HTTP header is not valid".to_string())
                    .error_response(),
            )));
        };

        // TODO: improve...
        // the problem is: req.extensions_mut().insert(auth);
        // we can either clone req or use a sync middleware
        // the question is: what is the cost of clonning req ?
        let mut service = self.service.clone();
        return Box::new(
            state
                .db
                .send(msg)
                .map_err(|_| BloomError::ActixMailbox)
                .from_err()
                .and_then(move |res: Result<_, BloomError>| match res {
                    Ok(auth) => {
                        req.extensions_mut().insert(auth);
                        return Either::A(service.call(req));
                    }
                    Err(e) => Either::B(ok(req.error_response(e.error_response()))),
                }),
        );
    }
}

// impl Middleware<api::State> for AuthMiddleware {
//     fn start(&self, req: &HttpRequest<api::State>) -> Result<Started, Error> {
//         let state = req.state().clone();

//         let auth_header = req.headers().get(header::AUTHORIZATION);
//         if auth_header.is_none() {
//             req.extensions_mut().insert(Auth{
//                 account: None,
//                 session: None,
//                 service: None,
//             });
//             return Ok(Started::Done);
//         }

//         let auth_header = auth_header.unwrap().to_str()
//             .map_err(|_| BloomError::Validation("Authorization HTTP header is not valid".to_string()))?;
//         let msg = if auth_header.starts_with("Basic ") {
//             extract_authorization_header(auth_header)
//                 .map_err(|_| BloomError::Validation("Authorization HTTP header is not valid".to_string()))?
//         } else if auth_header.starts_with("Secret ") {
//             extract_secret_header(auth_header)
//                 .map_err(|_| BloomError::Validation("Authorization HTTP header is not valid".to_string()))?
//         } else {
//             return Err(BloomError::Validation("Authorization HTTP header is not valid".to_string()).into());
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
//                         return Ok(None);
//                     },
//                     Err(e) => Err(e.into()),
//                 }
//             });

//         return Ok(Started::Future(Box::new(fut)));
//     }
// }

fn extract_authorization_header(value: &str) -> Result<CheckAuth, BloomError> {
    let parts: Vec<&str> = value.split("Basic ").collect();
    if parts.len() != 2 {
        return Err(BloomError::Validation(
            "Authorization HTTP header is not valid".to_string(),
        ));
    }

    let decoded = base64::decode(parts[1].trim()).map_err(|_| {
        BloomError::Validation("Authorization HTTP header is not valid".to_string())
    })?;
    let decoded = String::from_utf8(decoded).map_err(|_| {
        BloomError::Validation("Authorization HTTP header is not valid".to_string())
    })?;
    let parts: Vec<String> = decoded.split(':').map(String::from).collect();

    if parts.len() != 2 {
        return Err(BloomError::Validation(
            "Authorization HTTP header is not valid".to_string(),
        ));
    }

    let session_id = uuid::Uuid::parse_str(&parts[0]).map_err(|_| {
        BloomError::Validation("Authorization HTTP header is not valid".to_string())
    })?;
    return Ok(CheckAuth::Account(CheckAuthAccount {
        session_id,
        token: parts[1].clone(),
    }));
}

fn extract_secret_header(value: &str) -> Result<CheckAuth, BloomError> {
    let parts: Vec<&str> = value.split("Secret ").collect();
    if parts.len() != 2 {
        return Err(BloomError::Validation(
            "Authorization HTTP header is not valid".to_string(),
        ));
    }
    let parts: Vec<&str> = parts[1].split(':').collect();
    if parts.len() != 2 {
        return Err(BloomError::Validation(
            "Authorization HTTP header is not valid".to_string(),
        ));
    }

    let service = match parts[0] {
        "phaser" => Service::Phaser,
        "bitflow" => Service::Bitflow,
        _ => {
            return Err(BloomError::Validation(
                "Authorization HTTP header is not valid".to_string(),
            ))
        }
    };

    return Ok(CheckAuth::Service(CheckAuthService {
        service,
        secret: parts[1].to_string(),
    }));
}

pub trait GetRequestAuth {
    fn request_auth(&self) -> Auth;
}

impl GetRequestAuth for HttpRequest {
    fn request_auth(&self) -> Auth {
        return self
            .extensions()
            .get::<Auth>()
            .expect("retrieving request auth")
            .clone();
    }
}

#[derive(Clone, Debug)]
enum CheckAuth {
    Account(CheckAuthAccount),
    Service(CheckAuthService),
}

#[derive(Clone, Debug)]
struct CheckAuthAccount {
    pub session_id: uuid::Uuid,
    pub token: String,
}

#[derive(Clone, Debug)]
struct CheckAuthService {
    pub service: Service,
    pub secret: String,
}

impl Message for CheckAuth {
    type Result = Result<Auth, BloomError>;
}

impl Handler<CheckAuth> for DbActor {
    type Result = Result<Auth, BloomError>;

    fn handle(&mut self, msg: CheckAuth, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{accounts, sessions};
        use diesel::prelude::*;

        let conn = self.pool.get()?;

        let auth = match msg {
            CheckAuth::Account(msg) => {
                // find session + account
                let (session, account): (domain::Session, domain::Account) =
                    sessions::dsl::sessions
                        .filter(sessions::dsl::id.eq(msg.session_id))
                        .inner_join(accounts::table)
                        .first(&conn)
                        .map_err(|_| BloomError::Forbidden("Session is not valid".to_string()))?;

                // verify session token
                if !argon2id::verify_password(
                    &session.token_hash.as_str().into(),
                    msg.token.as_bytes(),
                ) {
                    return Err(BloomError::Validation(
                        "Authorization token is not valid".to_string(),
                    ));
                }

                Auth {
                    account: Some(account),
                    session: Some(session),
                    service: None,
                }
            }
            CheckAuth::Service(msg) => {
                let phaser_secret =
                    env::var("PHASER_SECRET").expect("error retrieving phaser secret");
                let bitflow_secret =
                    env::var("BITFLOW_SECRET").expect("error retrieving phaser secret");
                match (msg.service, msg.secret) {
                    (Service::Phaser, ref secret) if secret == &phaser_secret => Auth {
                        account: None,
                        session: None,
                        service: Some(Service::Phaser),
                    },
                    (Service::Bitflow, ref secret) if secret == &bitflow_secret => Auth {
                        account: None,
                        session: None,
                        service: Some(Service::Bitflow),
                    },
                    _ => {
                        return Err(BloomError::Validation(
                            "Authorization secret is not valid".to_string(),
                        ))
                    }
                }
            }
        };

        return Ok(auth);
    }
}

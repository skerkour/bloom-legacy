use actix_web::{
    HttpRequest, Result, Error,
    middleware::{Middleware, Started},
    http::header,
};
use futures::{Future};
use actix::{Message, Handler};
use crate::{
    db::DbActor,
    accounts::domain,
    api,
    error::KernelError,
};
use std::env;



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

impl Middleware<api::State> for AuthMiddleware {
    fn start(&self, req: &HttpRequest<api::State>) -> Result<Started, Error> {
        let state = req.state().clone();

        let auth_header = req.headers().get(header::AUTHORIZATION);
        if auth_header.is_none() {
            req.extensions_mut().insert(Auth{
                account: None,
                session: None,
                service: None,
            });
            return Ok(Started::Done);
        }


        let auth_header = auth_header.unwrap().to_str()
            .map_err(|_| KernelError::Validation("Authorization HTTP header is not valid".to_string()))?;
        let msg = if auth_header.starts_with("Basic ") {
            extract_authorization_header(auth_header)
                .map_err(|_| KernelError::Validation("Authorization HTTP header is not valid".to_string()))?
        } else if auth_header.starts_with("Secret ") {
            extract_secret_header(auth_header)
                .map_err(|_| KernelError::Validation("Authorization HTTP header is not valid".to_string()))?
        } else {
            return Err(KernelError::Validation("Authorization HTTP header is not valid".to_string()).into());
        };

        // TODO: improve...
        // the problem is: req.extensions_mut().insert(auth);
        // we can either clone req or use a sync middleware
        // the question is: what is the cost of clonning req ?
        let req = req.clone();

        let fut = state.db.send(msg)
            .from_err()
            .and_then(move |res| {
                match res {
                    Ok(auth) => {
                        req.extensions_mut().insert(auth);
                        return Ok(None);
                    },
                    Err(e) => Err(e.into()),
                }
            });

        return Ok(Started::Future(Box::new(fut)));
    }
}

fn extract_authorization_header(value: &str) -> Result<CheckAuth, KernelError> {
    let parts: Vec<&str> = value.split("Basic ").collect();
    if parts.len() != 2 {
        return Err(KernelError::Validation("Authorization HTTP header is not valid".to_string()));
    }

    let decoded = base64::decode(parts[1].trim())
        .map_err(|_| KernelError::Validation("Authorization HTTP header is not valid".to_string()))?;
    let decoded = String::from_utf8(decoded)
        .map_err(|_| KernelError::Validation("Authorization HTTP header is not valid".to_string()))?;
    let parts: Vec<String> = decoded.split(":").map(String::from).collect();

    if parts.len() != 2 {
        return Err(KernelError::Validation("Authorization HTTP header is not valid".to_string()));
    }

    let session_id = uuid::Uuid::parse_str(&parts[0])
        .map_err(|_| KernelError::Validation("Authorization HTTP header is not valid".to_string()))?;
    return Ok(CheckAuth::Account(CheckAuthAccount{
        session_id,
        token: parts[1].clone(),
    }));
}

fn extract_secret_header(value: &str) -> Result<CheckAuth, KernelError> {
    let parts: Vec<&str> = value.split("Secret ").collect();
    if parts.len() != 2 {
        return Err(KernelError::Validation("Authorization HTTP header is not valid".to_string()));
    }
    let parts: Vec<&str> = parts[1].split(":").collect();
    if parts.len() != 2 {
        return Err(KernelError::Validation("Authorization HTTP header is not valid".to_string()));
    }

    let service = match parts[0] {
        "phaser" => Service::Phaser,
        "bitflow" => Service::Bitflow,
        _ => return Err(KernelError::Validation("Authorization HTTP header is not valid".to_string())),
    };

    return Ok(CheckAuth::Service(CheckAuthService{
        service,
        secret: parts[1].to_string(),
    }));
}

pub trait GetRequestAuth {
    fn request_auth(&self) -> Auth;
}

impl<S> GetRequestAuth for HttpRequest<S> {
    fn request_auth(&self) -> Auth {
        return self.extensions().get::<Auth>().expect("retrieving request auth").clone();
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
    type Result = Result<Auth, KernelError>;
}

impl Handler<CheckAuth> for DbActor {
    type Result = Result<Auth, KernelError>;

    fn handle(&mut self, msg: CheckAuth, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            kernel_sessions,
            kernel_accounts,
        };
        use diesel::prelude::*;

        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        let auth = match msg {
            CheckAuth::Account(msg) => {
                // find session + account
                let (session, account): (domain::Session, domain::Account) = kernel_sessions::dsl::kernel_sessions
                    .filter(kernel_sessions::dsl::id.eq(msg.session_id))
                    .filter(kernel_sessions::dsl::deleted_at.is_null())
                    .inner_join(kernel_accounts::table)
                    .first(&conn)?;

                // verify session token
                if !bcrypt::verify(&msg.token, &session.token)
                    .map_err(|_| KernelError::Validation("Authorization HTTP header is not valid".to_string()))? {
                    return Err(KernelError::Validation("Authorization token is not valid".to_string()));
                }

                Auth{
                    account: Some(account),
                    session: Some(session),
                    service: None,
                }
            },
            CheckAuth::Service(msg) => {
                let phaser_secret = env::var("PHASER_SECRET").expect("error retrieving phaser secret");
                let bitflow_secret = env::var("BITFLOW_SECRET").expect("error retrieving phaser secret");
                match (msg.service, msg.secret) {
                    (Service::Phaser, ref secret) if secret == &phaser_secret => {
                        Auth{
                            account: None,
                            session: None,
                            service: Some(Service::Phaser),
                        }
                    },
                    (Service::Bitflow, ref secret) if secret == &bitflow_secret => {
                        Auth{
                            account: None,
                            session: None,
                            service: Some(Service::Bitflow),
                        }
                    }
                    _ => return Err(KernelError::Validation("Authorization secret is not valid".to_string())),
                }
            }
        };

        return Ok(auth);
    }
}

use actix_web::{
    HttpRequest, HttpResponse, Result, Error,
    middleware::{Middleware, Started},
    http::header,
    error::ResponseError,
    FutureResponse, AsyncResponder,
};
use futures::{Future, future};
use actix::{Message, Handler};
use crate::{
    db::DbActor,
    services::account::domain,
    api,
    services::account::domain::pending_account,
    services::account::notifications::emails::send_account_verification_code,
    services::common::events::EventMetadata,
    config::Config,
    error::KernelError,
};
use base64::{encode, decode};


#[derive(Debug, Clone)]
pub struct Auth {
    pub account: Option<domain::Account>,
    pub session: Option<domain::Session>,
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
            });
            return Ok(Started::Done);
        }

        let req = req.clone(); // TODO: improve...
        println!("auth header: {:?}", auth_header);

        let auth_header = match auth_header.unwrap().to_str() {
            Ok(x) => x,
            Err(_) => return Ok(Started::Response(
                api::Error::from(KernelError::Validation("Authorization HTTP header is not valid".to_string())).error_response()
            )),
        };
        let msg = match extract_authorization_header(auth_header) {
            Ok(x) => x,
            Err(_) => return Ok(Started::Response(
                api::Error::from(KernelError::Validation("Authorization HTTP header is not valid".to_string())).error_response()
            )),
        };

        //TODO: split header

        let fut = state.db.send(msg)
            .from_err()
            .and_then(move |res| {
                match res {
                    Ok(auth) => {
                        req.extensions_mut().insert(auth);
                        return Ok(None);
                    },
                    Err(e) => Err(api::Error::from(e).into()),
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
    return Ok(CheckAuth{
        session_id,
        token: parts[1].clone(),
    });
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
struct CheckAuth {
    pub session_id: uuid::Uuid,
    pub token: String,
}

impl Message for CheckAuth {
    type Result = Result<Auth, KernelError>;
}

impl Handler<CheckAuth> for DbActor {
    type Result = Result<Auth, KernelError>;

    fn handle(&mut self, msg: CheckAuth, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            account_sessions,
            account_accounts,
        };
        use diesel::prelude::*;

        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        // ind session + account
        let (session, account): (domain::Session, domain::Account) = account_sessions::dsl::account_sessions
                .filter(account_sessions::dsl::id.eq(msg.session_id))
                .filter(account_sessions::dsl::deleted_at.is_null())
                .inner_join(account_accounts::table)
                .first(&conn)?;

        // verify session token
        if !bcrypt::verify(&msg.token, &session.token)
            .map_err(|_| KernelError::Validation("Authorization HTTP header is not valid".to_string()))? {
            return Err(KernelError::Validation("Authorization token is not valid".to_string()));
        }

        return Ok(Auth{
            account: Some(account),
            session: Some(session),
        });
    }
}

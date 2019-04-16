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


        let auth_header = auth_header.unwrap().to_str()
            .map_err(|_| KernelError::Validation("Authorization HTTP header is not valid".to_string()))?;
        let msg = extract_authorization_header(auth_header)
            .map_err(|_| KernelError::Validation("Authorization HTTP header is not valid".to_string()))?;


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
            kernel_sessions,
            kernel_accounts,
        };
        use diesel::prelude::*;

        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

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

        return Ok(Auth{
            account: Some(account),
            session: Some(session),
        });
    }
}

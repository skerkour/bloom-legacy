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


/// Auth
#[derive(Debug, Clone, PartialEq)]
pub struct Auth;

impl Middleware<api::State> for Auth {
    fn start(&self, req: &HttpRequest<api::State>) -> Result<Started, Error> {
        let state = req.state().clone();

        let auth_header = req.headers().get(header::AUTHORIZATION);
        if auth_header.is_none() {
            return Ok(Started::Done);
        }

        println!("auth header: {:?}", auth_header);
        //TODO: split header

        let fut = state.db
            .send(CheckSession{
                session_id: "aa".to_string(),
                token: "bb".to_string(),
            })
            .from_err()
            .and_then(|res| {
                match res {
                    Ok(_) => Ok(None),
                    Err(e) => Err(api::Error::from(e).into()),
                }
            });
        return Ok(Started::Future(Box::new(fut)));
    }
}


#[derive(Clone, Debug)]
struct CheckSession {
    pub session_id: String,
    pub token: String,
}

impl Message for CheckSession {
    type Result = Result<(domain::Account, domain::Session), KernelError>;
}

impl Handler<CheckSession> for DbActor {
    type Result = Result<(domain::Account, domain::Session), KernelError>;

    fn handle(&mut self, msg: CheckSession, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            account_sessions,
            account_sessions_events,
            account_accounts,
            account_accounts_events,
        };
        use diesel::prelude::*;

        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        // TODO: find session + account
        // verify session token

        return Err(KernelError::Validation("checking session".to_string()));
    }
}

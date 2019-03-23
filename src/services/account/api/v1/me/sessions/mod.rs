mod revoke;


use crate::{
    api,
    log::macros::*,
    services::account::controllers,
    services::account::api::v1::models,
    api::middlewares::{
        GetRequestLogger,
        GetRequestId,
        GetRequestAuth,
    },
    error::KernelError,
};
use futures::future::Future;
use actix_web::{
    FutureResponse, AsyncResponder, HttpResponse, HttpRequest, ResponseError, Json,
};
use futures::future;


pub use revoke::me_sessions_revoke_post;

pub fn me_sessions_get(req: &HttpRequest<api::State>) -> FutureResponse<HttpResponse> {
    let state = req.state().clone();
    let logger = req.logger();
    let auth = req.request_auth();

    if auth.session.is_none() || auth.account.is_none() {
        return future::result(Ok(api::Error::from(KernelError::Unauthorized("Authentication required".to_string())).error_response()))
        .responder();
    }

    return state.db
    .send(controllers::FindAccountSessions{
        account_id: auth.account.expect("unwrapping non non account").id,
    })
    .from_err()
    .and_then(move |sessions| {
        match sessions {
            Ok(sessions) => {
                let sessions: Vec<models::Session> = sessions.into_iter().map(|session| {
                    models::Session{
                        id: session.id,
                        created_at: session.created_at,
                        ip: session.ip,
                        location: session.location,
                        device: session.device,
                    }
                }).collect();
                let res = api::Response::data(sessions);
                Ok(HttpResponse::Ok().json(&res))
            },
            Err(err) => Err(err),
        }
    })
    .from_err()
    .map_err(move |err: KernelError| {
        slog_error!(logger, "{}", err);
        return api::Error::from(err);
    })
    .from_err()
    .responder();
}

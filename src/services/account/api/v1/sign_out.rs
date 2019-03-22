use crate::{
    api,
    services::account::api::v1::models,
    log::macros::*,
    api::middlewares::logger::GetRequestLogger,
    services::account::controllers,
    api::middlewares::request_id::GetRequestID,
    api::middlewares::auth::GetRequestAuth,
    error::KernelError,
};
use std::time::Duration;
use futures::future::Future;
use actix_web::{
    FutureResponse, AsyncResponder, HttpResponse, Json, HttpRequest, ResponseError,
};
use futures::future;
use rand::Rng;
use failure::Fail;


pub fn sign_out_post((_, req): (Json<models::SignOutBody>, HttpRequest<api::State>))
-> FutureResponse<HttpResponse> {
    let mut rng = rand::thread_rng();
    let state = req.state().clone();
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;
    let config = state.config.clone();

    if auth.session.is_none() || auth.account.is_none() {
        return future::result(Ok(api::Error::from(KernelError::Unauthorized("auth required".to_string())).error_response()))
            .responder();
    }

    return state.db
    .send(controllers::SignOut{
        actor: auth.account.unwrap(),
        session: auth.session.unwrap(),
        request_id,
    })
    .and_then(move |_| {
        let res = models::SignOutResponse{};
        let res = api::Response::data(res);
        Ok(HttpResponse::Created().json(&res))
    })
    .from_err() // MailboxError to KernelError
    .map_err(move |err: KernelError| {
        slog_error!(logger, "{}", err);
        return api::Error::from(err);
    })
    .from_err()
    .responder();
}

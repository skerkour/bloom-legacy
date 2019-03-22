use crate::{
    api,
    services::account::api::v1::models,
    log::macros::*,
    services::account::controllers,
    api::middlewares::{
        GetRequestLogger,
        GetRequestId,
        GetRequestAuth,
    },
    error::KernelError,
};
use futures::future::Future;
use actix_web::{
    FutureResponse, AsyncResponder, HttpResponse, HttpRequest, ResponseError,
};
use futures::future;


pub fn sign_out_post(req: &HttpRequest<api::State>)
-> FutureResponse<HttpResponse> {
    let state = req.state().clone();
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;

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
        Ok(HttpResponse::Ok().json(&res))
    })
    .from_err() // MailboxError to KernelError
    .map_err(move |err: KernelError| {
        slog_error!(logger, "{}", err);
        return api::Error::from(err);
    })
    .from_err()
    .responder();
}

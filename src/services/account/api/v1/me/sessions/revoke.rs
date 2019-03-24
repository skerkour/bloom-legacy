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
    FutureResponse, AsyncResponder, HttpResponse, HttpRequest, ResponseError, Path,
};
use futures::future;


pub fn post((session_id, req): (Path<(String)>, HttpRequest<api::State>)) -> FutureResponse<HttpResponse> {
    let state = req.state().clone();
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;

    if auth.session.is_none() || auth.account.is_none() {
        return future::result(Ok(api::Error::from(KernelError::Unauthorized("Authentication required".to_string())).error_response()))
            .responder();
    }

    let session_id = match uuid::Uuid::parse_str(&session_id) {
        Ok(x) => x,
        Err(_) => return future::result(Ok(api::Error::from(KernelError::Validation("session_id is not valid".to_string())).error_response()))
            .responder(),
    };

    return state.db
    .send(controllers::RevokeSession{
        actor: auth.account.unwrap(),
        session_id: session_id,
        request_id: request_id,
        current_session_id: auth.session.unwrap().id,
    })
    .and_then(move |_| {
        let res = models::EmptyData{};
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

use crate::{
    api,
    accounts::api::v1::models,
    log::macros::*,
    accounts::controllers,
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
use futures::future::IntoFuture;
use rand::Rng;
use std::time::Duration;



pub fn put((email_data, req): (Json<models::UpdateEmailBody>, HttpRequest<api::State>))
-> FutureResponse<HttpResponse> {
    let state = req.state().clone();
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;
    let config = state.config.clone();
    let mut rng = rand::thread_rng();

    if auth.session.is_none() || auth.account.is_none() {
        return future::result(Ok(KernelError::Unauthorized("Authentication required".to_string()).error_response()))
            .responder();
    }

    // random sleep to prevent bruteforce and sidechannels attacks
    return tokio_timer::sleep(Duration::from_millis(rng.gen_range(250, 350))).into_future()
    .from_err()
    .and_then(move |_|
        state.db
        .send(controllers::UpdateEmail{
            account: auth.account.expect("unwraping auth account"),
            email: email_data.email.clone(),
            config,
            request_id,
            session_id: auth.session.expect("unwraping non none session").id,
        }).flatten()
    )
    .and_then(move |pending_email| {
        let res = models::UpdateEmailResponse{
            id: pending_email.id,
        };
        let res = api::Response::data(res);
        Ok(HttpResponse::Ok().json(&res))
    })
    .map_err(move |err: KernelError| {
        slog_error!(logger, "{}", err);
        return err;
    })
    .from_err()
    .responder();
}

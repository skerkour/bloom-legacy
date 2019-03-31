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



pub fn post((email_data, req): (Json<models::VerifyEmailBody>, HttpRequest<api::State>))
-> FutureResponse<HttpResponse> {
    let state = req.state().clone();
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;
    let mut rng = rand::thread_rng();

    if auth.session.is_none() || auth.account.is_none() {
        return future::result(Ok(KernelError::Unauthorized("Authentication required".to_string()).error_response()))
            .responder();
    }

    // random sleep to prevent bruteforce and sidechannels attacks
    return tokio_timer::sleep(Duration::from_millis(rng.gen_range(350, 550))).into_future()
    .from_err()
    .and_then(move |_|
        state.db
        .send(controllers::VerifyEmail{
            account: auth.account.expect("unwraping non none account"),
            id: email_data.id,
            code: email_data.code.clone(),
            request_id,
            session_id: auth.session.expect("unwraping non none session").id,
        }).flatten()
    )
    .and_then(move |account| {
        let res = models::MeResponse{
            id: account.id,
            created_at: account.created_at,
            first_name: account.first_name,
            last_name: account.last_name,
            username: account.username,
            email: account.email,
            avatar_url: account.avatar_url,
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

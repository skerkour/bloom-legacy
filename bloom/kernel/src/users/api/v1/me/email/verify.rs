use crate::{
    api,
    users::api::v1::models,
    log::macros::*,
    users::controllers,
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

    if auth.session.is_none() || auth.user.is_none() {
        return future::result(Ok(KernelError::Unauthorized("Authentication required".to_string()).error_response()))
            .responder();
    }

    // random sleep to prevent bruteforce and sidechannels attacks
    return tokio_timer::sleep(Duration::from_millis(rng.gen_range(350, 550))).into_future()
    .from_err()
    .and_then(move |_|
        state.db
        .send(controllers::VerifyEmail{
            user: auth.user.expect("unwraping non none user"),
            id: email_data.id,
            code: email_data.code.clone(),
            request_id,
            session_id: auth.session.expect("unwraping non none session").id,
        }).flatten()
    )
    .and_then(move |user| {
        let res = models::MeResponse{
            id: user.id,
            created_at: user.created_at,
            first_name: user.first_name,
            last_name: user.last_name,
            username: user.username,
            email: user.email,
            avatar_url: user.avatar_url,
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

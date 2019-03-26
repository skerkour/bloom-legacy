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
    FutureResponse, AsyncResponder, HttpResponse, HttpRequest, ResponseError, Json,
};
use futures::future::IntoFuture;
use rand::Rng;
use std::time::Duration;



pub fn post((data, req): (Json<models::PasswordResetRequestBody>, HttpRequest<api::State>))
-> FutureResponse<HttpResponse> {
    let state = req.state().clone();
    let logger = req.logger();
    let request_id = req.request_id().0;
    let mut rng = rand::thread_rng();
    let config = state.config.clone();

    // random sleep to prevent bruteforce and sidechannels attacks
    return tokio_timer::sleep(Duration::from_millis(rng.gen_range(250, 350))).into_future()
    .from_err()
    .and_then(move |_|
        state.db
        .send(controllers::RequestPasswordReset{
            config,
            email_or_username: data.email_or_username.clone(),
            request_id,
        }).flatten()
    )
    .and_then(move |_| {
        let res = api::Response::data(models::NoData{});
        Ok(HttpResponse::Ok().json(&res))
    })
    .map_err(move |err: KernelError| {
        slog_error!(logger, "{}", err);
        return api::Error::from(err);
    })
    .from_err()
    .responder();
}

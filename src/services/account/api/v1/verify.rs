use crate::{
    api,
    services::account::api::v1::models,
    log::macros::*,
    services::account::controllers,
    api::middlewares::{
        GetRequestLogger,
        GetRequestId,
    },
};
use std::time::Duration;
use futures::future::Future;
use actix_web::{
    FutureResponse, HttpResponse, Json, HttpRequest, AsyncResponder,
};
use futures::future::IntoFuture;
use rand::Rng;



pub fn post((verify_data, req): (Json<models::VerifyBody>, HttpRequest<api::State>))
-> FutureResponse<HttpResponse> {
    let mut rng = rand::thread_rng();
    let state = req.state().clone();
    let logger = req.logger();
    let request_id = req.request_id();

    // random sleep to prevent bruteforce and sidechannels attacks
    return tokio_timer::sleep(Duration::from_millis(rng.gen_range(500, 650))).into_future()
    .from_err()
    .and_then(move |_|
        state.db
        .send(controllers::Verify{
            id: verify_data.id.clone(),
            code: verify_data.code.clone(),
            request_id: request_id.0,
        }).flatten()
    )
    .and_then(move |_| {
        let res = api::Response::data(models::VerifyResponse{is_valid: true});
        Ok(HttpResponse::Ok().json(&res))
    })
    .map_err(move |err| {
        slog_error!(logger, "{}", err);
        return api::Error::from(err);
    })
    .from_err()
    .responder();
}

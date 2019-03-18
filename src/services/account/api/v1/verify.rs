use crate::{
    api,
    services::account::api::v1::models,
    log::macros::*,
    api::middlewares::logger::GetRequestLogger,
};
use std::time::Duration;
use futures::future::Future;
use actix_web::{
    FutureResponse, HttpResponse, Json, HttpRequest, AsyncResponder,
};
use crate::services::account::domain::{
    pending_account,
};
use futures::future::IntoFuture;
use rand::Rng;



// pub fn verify_post(verify_data: Json<models::VerifyBody>, state: State<api::State>) -> FutureResponse<HttpResponse> {
pub fn verify_post((verify_data, req): (Json<models::VerifyBody>, HttpRequest<api::State>))
-> FutureResponse<HttpResponse> {
    let mut rng = rand::thread_rng();
    let state = req.state().clone();
    let logger = req.logger();

    return tokio_timer::sleep(Duration::from_millis(rng.gen_range(700, 900))).into_future()
    .from_err()
    .and_then(move |_|
        state.db
        .send(pending_account::Verify{
            id: verify_data.id.clone(),
            code: verify_data.code.clone(),
        }).flatten()
    )
    .and_then(move |is_valid| {
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

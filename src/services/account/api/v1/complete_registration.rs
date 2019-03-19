use crate::{
    api,
    services::account::api::v1::models,
    log::macros::*,
    api::middlewares::logger::GetRequestLogger,
    services::account::controllers,
};
use std::time::Duration;
use futures::future::Future;
use actix_web::{
    FutureResponse, AsyncResponder, HttpResponse, Json, HttpRequest,
};
use crate::services::account::domain::{
    pending_account,
};
use futures::future::IntoFuture;
use rand::Rng;
use failure::Fail;


pub fn complete_registration_post((registration_data, req): (Json<models::CompleteRegistrationBody>, HttpRequest<api::State>))
-> FutureResponse<HttpResponse> {
    let mut rng = rand::thread_rng();
    let state = req.state().clone();
    let logger = req.logger();

    // random sleep to prevent bruteforce and sidechannels attacks
    return tokio_timer::sleep(Duration::from_millis(rng.gen_range(400, 650))).into_future()
    .from_err()
    .and_then(move |_|
        state.db
        .send(controllers::CompleteRegistration{
            id: registration_data.id.clone(),
            code: registration_data.code.clone(),
            username: registration_data.username.clone(),
        }).flatten()
    )
    .and_then(move |session| {
        let res = api::Response::data(models::CompleteRegistrationResponse{
            id: "a".to_string(),
            token: "b".to_string(),
        });
        Ok(HttpResponse::Created().json(&res))
    })
    .map_err(move |err| {
        slog_error!(logger, "{}", err);
        return api::Error::from(err);
    })
    .from_err()
    .responder();
}

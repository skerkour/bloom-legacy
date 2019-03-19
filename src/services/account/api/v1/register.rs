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
use futures::future::IntoFuture;
use rand::Rng;
use failure::Fail;


pub fn register_post((register_data, req): (Json<models::RegisterBody>, HttpRequest<api::State>))
-> FutureResponse<HttpResponse> {
    let mut rng = rand::thread_rng();
    let state = req.state().clone();
    let logger = req.logger();

    // random sleep to prevent bruteforce and sidechannels attacks
    return tokio_timer::sleep(Duration::from_millis(rng.gen_range(200, 400))).into_future()
    .from_err()
    .and_then(move |_|
        state.db
        .send(controllers::Register{
            first_name: register_data.first_name.clone(),
            last_name: register_data.last_name.clone(),
            email: register_data.email.clone(),
            password: register_data.password.clone(),
            config: state.config.clone(),
        }).flatten()
    )
    .and_then(move |pending_account| {
        let res = models::RegisterResponse::from(pending_account);
        let res = api::Response::data(res);
        Ok(HttpResponse::Created().json(&res))
    })
    .map_err(move |err| {
        slog_error!(logger, "{}", err);
        return api::Error::from(err);
    })
    .from_err()
    .responder();
}

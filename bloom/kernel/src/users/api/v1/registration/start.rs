use crate::{
    api,
    users::api::v1::models,
    log::macros::*,
    users::controllers,
    api::middlewares::{
        GetRequestLogger,
        GetRequestId,
    },
};
use std::time::Duration;
use futures::future::Future;
use actix_web::{
    FutureResponse, AsyncResponder, HttpResponse, Json, HttpRequest,
};
use futures::future::IntoFuture;
use rand::Rng;


pub fn post((register_data, req): (Json<models::StartRegistrationBody>, HttpRequest<api::State>))
-> FutureResponse<HttpResponse> {
    let mut rng = rand::thread_rng();
    let state = req.state().clone();
    let logger = req.logger();
    let request_id = req.request_id().0;
    let config = state.config.clone();

    // random sleep to prevent bruteforce and sidechannels attacks
    return tokio_timer::sleep(Duration::from_millis(rng.gen_range(200, 400))).into_future()
    .from_err()
    .and_then(move |_|
        state.db
        .send(controllers::StartRegistration{
            first_name: register_data.first_name.clone(),
            last_name: register_data.last_name.clone(),
            email: register_data.email.clone(),
            password: register_data.password.clone(),
            config,
            request_id,
        }).flatten()
    )
    .and_then(move |pending_user| {
        let res = models::StartRegistrationResponse{
            id: pending_user.id,
        };
        let res = api::Response::data(res);
        Ok(HttpResponse::Created().json(&res))
    })
    .map_err(move |err| {
        slog_error!(logger, "{}", err);
        return err;
    })
    .from_err()
    .responder();
}

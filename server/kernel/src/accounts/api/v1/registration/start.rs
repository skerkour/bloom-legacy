use crate::{
    api,
    accounts::api::v1::models,
    log::macros::*,
    accounts::controllers,
    api::middlewares::{
        GetRequestLogger,
        GetRequestId,
    },
};
use std::time::Duration;
use futures::future::Future;
use actix_web::{
    web, Error, HttpRequest, HttpResponse,
};
use futures::future::IntoFuture;
use rand::Rng;


pub fn post(register_data: web::Json<models::StartRegistrationBody>, state: web::Data<api::State>, req: HttpRequest)
-> impl Future<Item = HttpResponse, Error = Error> {
    let mut rng = rand::thread_rng();
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
    .and_then(move |pending_account| {
        let res = models::StartRegistrationResponse{
            id: pending_account.id,
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

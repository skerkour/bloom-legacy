use crate::{
    api,
    myaccount::api::v1::models,
    log::macros::*,
    myaccount::controllers,
    api::middlewares::{
        GetRequestLogger,
        GetRequestId,
        GetRequestAuth,
    },
    KernelError,
};
use std::time::Duration;
use futures::{
    future::{
        IntoFuture,
        Future,
        ok,
        Either,
    },
};
use actix_web::{
    web, Error, HttpRequest, HttpResponse, ResponseError,
};
use rand::Rng;


pub fn post(register_data: web::Json<models::StartRegistrationBody>, state: web::Data<api::State>, req: HttpRequest)
-> impl Future<Item = HttpResponse, Error = Error> {
    let mut rng = rand::thread_rng();
    let logger = req.logger();
    let request_id = req.request_id().0;
    let config = state.config.clone();
    let auth = req.request_auth();

    if auth.session.is_some() || auth.account.is_some() || auth.service.is_some() {
        return Either::A(ok(KernelError::Unauthorized("Must not be authenticated".to_string()).error_response()));
    }

    // random sleep to prevent bruteforce and sidechannels attacks
    return Either::B(
        tokio_timer::sleep(Duration::from_millis(rng.gen_range(200, 400))).into_future()
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
        .from_err()
        .and_then(move |pending_account| {
            let res = models::StartRegistrationResponse{
                id: pending_account.id,
            };
            let res = api::Response::data(res);
            ok(HttpResponse::Created().json(&res))
        })
        .map_err(move |err: KernelError| {
            slog_error!(logger, "{}", err);
            return err.error_response();
        })
        .from_err()
    );
}

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


pub fn post(verify_data: web::Json<models::VerifyPendingAccountBody>, state: web::Data<api::State>, req: HttpRequest)
-> impl Future<Item = HttpResponse, Error = Error> {
    let mut rng = rand::thread_rng();
    let logger = req.logger();
    let request_id = req.request_id();
    let auth = req.request_auth();

    if auth.session.is_some() || auth.account.is_some() || auth.service.is_some() {
        return Either::A(ok(KernelError::Unauthorized("Must not be authenticated".to_string()).error_response()));
    }

    // random sleep to prevent bruteforce and sidechannels attacks
    return Either::B(
        tokio_timer::sleep(Duration::from_millis(rng.gen_range(500, 650))).into_future()
        .from_err()
        .and_then(move |_|
            state.db
            .send(controllers::VerifyPendingAccount{
                id: verify_data.id,
                code: verify_data.code.clone(),
                request_id: request_id.0,
            }).flatten()
        )
        .from_err()
        .and_then(move |_| {
        ok(HttpResponse::Ok().json(api::Response::data(api::NoData{})))
        })
        .map_err(move |err: KernelError| {
            slog_error!(logger, "{}", err);
            return err.error_response();
        })
        .from_err()
    );
}

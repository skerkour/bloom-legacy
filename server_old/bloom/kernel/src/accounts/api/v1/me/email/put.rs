use crate::{
    api,
    api::middlewares::{GetRequestAuth, GetRequestId, GetRequestLogger},
    error::KernelError,
    log::macros::*,
    accounts::api::v1::models,
    accounts::controllers,
};
use actix_web::{web, Error, HttpRequest, HttpResponse, ResponseError};
use futures::future::{ok, Either, Future, IntoFuture};
use rand::Rng;
use std::time::Duration;

pub fn put(
    email_data: web::Json<models::UpdateEmailBody>,
    state: web::Data<api::State>,
    req: HttpRequest,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;
    let config = state.config.clone();
    let mut rng = rand::thread_rng();

    if auth.session.is_none() || auth.account.is_none() {
        return Either::A(ok(KernelError::Unauthorized(
            "Authentication required".to_string(),
        )
        .error_response()));
    }

    // random sleep to prevent bruteforce and sidechannels attacks
    return Either::B(
        tokio_timer::sleep(Duration::from_millis(rng.gen_range(250, 350)))
            .into_future()
            .from_err()
            .and_then(move |_| {
                state
                    .db
                    .send(controllers::UpdateEmail {
                        account: auth.account.expect("unwraping auth account"),
                        email: email_data.email.clone(),
                        config,
                        request_id,
                        session_id: auth.session.expect("unwraping non none session").id,
                    })
                    .flatten()
            })
            .from_err()
            .and_then(move |pending_email| {
                let res = models::UpdateEmailResponse {
                    id: pending_email.id,
                };
                let res = api::Response::data(res);
                ok(HttpResponse::Ok().json(&res))
            })
            .map_err(move |err: KernelError| {
                slog_error!(logger, "{}", err);
                return err.error_response();
            })
            .from_err(),
    );
    // .map_err(move |err: KernelError| {
    //     slog_error!(logger, "{}", err);
    //     return err;
    // })
    // .from_err()
    // .responder();
}

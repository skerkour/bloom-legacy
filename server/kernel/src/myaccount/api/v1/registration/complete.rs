use crate::{
    api,
    api::middlewares::{GetRequestAuth, GetRequestId, GetRequestLogger},
    log::macros::*,
    myaccount::api::v1::models,
    myaccount::controllers,
    utils, KernelError,
};
use actix_web::{web, Error, HttpRequest, HttpResponse, ResponseError};
use futures::future::{ok, Either, Future, IntoFuture};
use rand::Rng;
use std::time::Duration;

pub fn post(
    registration_data: web::Json<models::CompleteRegistrationBody>,
    state: web::Data<api::State>,
    req: HttpRequest,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let mut rng = rand::thread_rng();
    let logger = req.logger();
    let config = state.config.clone();
    let request_id = req.request_id().0;
    let auth = req.request_auth();
    let connection_info = req.connection_info();
    let remote = connection_info.remote();

    if auth.session.is_some() || auth.account.is_some() || auth.service.is_some() {
        return Either::A(ok(KernelError::Unauthorized(
            "Must not be authenticated".to_string(),
        )
        .error_response()));
    }

    let ip = match remote {
        Some(ref remote) => remote
            .split(':')
            .nth(0)
            .expect("Error accessing session ip")
            .to_string(),
        _ => "".to_string(),
    };

    // random sleep to prevent bruteforce and sidechannels attacks
    return Either::B(
        tokio_timer::sleep(Duration::from_millis(rng.gen_range(400, 650)))
            .into_future()
            .from_err()
            .and_then(move |_| {
                state
                    .db
                    .send(controllers::CompleteRegistration {
                        id: registration_data.id,
                        username: registration_data.username.clone(),
                        config,
                        request_id,
                        ip,
                    })
                    .flatten()
            })
            .from_err()
            .and_then(|(session, token)| {
                let res = api::Response::data(models::CompleteRegistrationResponse {
                    id: session.id,
                    token: utils::encode_session(&session.id.to_string(), &token),
                });
                ok(HttpResponse::Created().json(&res))
            })
            .map_err(move |err: KernelError| {
                slog_error!(logger, "{}", err);
                return err.error_response();
            })
            .from_err(),
    );
}

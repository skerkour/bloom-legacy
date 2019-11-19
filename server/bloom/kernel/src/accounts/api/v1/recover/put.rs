use crate::{
    api,
    api::middlewares::{GetRequestAuth, GetRequestId, GetRequestLogger},
    error::KernelError,
    log::macros::*,
    accounts::api::v1::models,
    accounts::controllers,
    utils,
};
use actix_web::{web, Error, HttpRequest, HttpResponse, ResponseError};
use futures::future::{ok, Future, IntoFuture};
use rand::Rng;
use std::time::Duration;

pub fn put(
    password_data: web::Json<models::ResetPassowrdBody>,
    state: web::Data<api::State>,
    req: HttpRequest,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;
    let mut rng = rand::thread_rng();
    let config = state.config.clone();
    let connection_info = req.connection_info();
    let remote = connection_info.remote();

    let session_id = match auth.session {
        Some(ref session) => Some(session.id),
        None => None,
    };

    let ip = match remote {
        Some(ref remote) => remote
            .split(':')
            .nth(0)
            .expect("Error accessing session ip")
            .to_string(),
        _ => "".to_string(),
    };

    // random sleep to prevent bruteforce and sidechannels attacks
    return tokio_timer::sleep(Duration::from_millis(rng.gen_range(250, 350)))
        .into_future()
        .from_err()
        .and_then(move |_| {
            state
                .db
                .send(controllers::ResetPassword {
                    reset_password_id: password_data.id,
                    token: password_data.token.clone(),
                    new_password: password_data.new_password.clone(),
                    config,
                    request_id,
                    session_id,
                    ip,
                })
                .flatten()
        })
        .from_err()
        .and_then(move |(session, token)| {
            let res = api::Response::data(models::SignInResponse {
                token: utils::encode_session(&session.id.to_string(), &token),
                id: session.id,
            });
            ok(HttpResponse::Ok().json(&res))
        })
        .map_err(move |err: KernelError| {
            slog_error!(logger, "{}", err);
            return err.error_response();
        })
        .from_err();
}

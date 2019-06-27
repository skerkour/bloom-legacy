use crate::{
    api,
    api::middlewares::{GetRequestAuth, GetRequestId, GetRequestLogger},
    error::KernelError,
    log::macros::*,
    myaccount::api::v1::models,
    myaccount::controllers,
    utils,
};
use actix_web::{web, Error, HttpRequest, HttpResponse, ResponseError};
use futures::future::{ok, Either, Future, IntoFuture};
use rand::Rng;
use std::time::Duration;

pub fn post(
    sign_in_data: web::Json<models::SignInBody>,
    state: web::Data<api::State>,
    req: HttpRequest,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;
    let mut rng = rand::thread_rng();
    let connection_info = req.connection_info();
    let remote = connection_info.remote().clone();

    if auth.session.is_some() || auth.account.is_some() || auth.service.is_some() {
        return Either::A(ok(KernelError::Unauthorized(
            "Must not be authenticated".to_string(),
        )
        .error_response()));
    }

    let session_ip = match remote {
        Some(ref remote) => remote
            .split(":")
            .into_iter()
            .nth(0)
            .expect("Error accessing session ip")
            .to_string(),
        _ => "".to_string(),
    };

    return Either::B(
        tokio_timer::sleep(Duration::from_millis(rng.gen_range(400, 600)))
            .into_future()
            .from_err()
            .and_then(move |_| {
                state
                    .db
                    .send(controllers::SignIn {
                        username: sign_in_data.username.clone(),
                        password: sign_in_data.password.clone(),
                        session_ip,
                        request_id,
                    })
                    .flatten()
            })
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
            .from_err(),
    );
}

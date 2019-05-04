use crate::{
    api,
    accounts::api::v1::models,
    log::macros::*,
    accounts::controllers,
    api::middlewares::{
        GetRequestLogger,
        GetRequestId,
        GetRequestAuth,
    },
    error::KernelError,
    utils,
};
use futures::{
    future::{
        IntoFuture,
        Either,
        ok,
        Future,
    },
};
use actix_web::{
    web, Error, HttpRequest, HttpResponse, ResponseError,
};
use std::time::Duration;
use rand::Rng;


pub fn post(sign_in_data: web::Json<models::SignInBody>, state: web::Data<api::State>, req: HttpRequest)
-> impl Future<Item = HttpResponse, Error = Error> {
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;
    let mut rng = rand::thread_rng();

   if auth.session.is_none() || auth.account.is_none() {
        return Either::A(ok(KernelError::Unauthorized("Authentication required".to_string()).error_response()));
    }

    return Either::B(
        tokio_timer::sleep(Duration::from_millis(rng.gen_range(400, 600))).into_future()
        .from_err()
        .and_then(move |_|
            state.db
            .send(controllers::SignIn{
                username: sign_in_data.username.clone(),
                password: sign_in_data.password.clone(),
                request_id,
            }).flatten()
        )
        .and_then(move |(session, token)| {
            let res = api::Response::data(models::SignInResponse{
                token: utils::encode_session(&session.id.to_string(), &token),
                id: session.id,
            });
            ok(HttpResponse::Ok().json(&res))
        })
        .map_err(move |err: KernelError| {
            slog_error!(logger, "{}", err);
            return err;
        })
        .from_err()
    );
}

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
    error::KernelError,
};
use rand::Rng;
use std::time::Duration;
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


pub fn post(email_data: web::Json<models::VerifyEmailBody>, state: web::Data<api::State>, req: HttpRequest)
-> impl Future<Item = HttpResponse, Error = Error> {
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;
    let mut rng = rand::thread_rng();

    if auth.session.is_none() || auth.account.is_none() {
        return Either::A(ok(KernelError::Unauthorized("Authentication required".to_string()).error_response()));
    }

    // random sleep to prevent bruteforce and sidechannels attacks
    return Either::B(
        tokio_timer::sleep(Duration::from_millis(rng.gen_range(350, 550))).into_future()
        .from_err()
        .and_then(move |_|
            state.db
            .send(controllers::VerifyEmail{
                account: auth.account.expect("unwraping non none account"),
                id: email_data.id,
                code: email_data.code.clone(),
                request_id,
                session_id: auth.session.expect("unwraping non none session").id,
            }).flatten()
        )
        .from_err()
        .and_then(move |account| {
            let res = models::MeResponse{
                id: account.id,
                created_at: account.created_at,
                first_name: account.first_name,
                last_name: account.last_name,
                username: account.username,
                email: account.email,
                avatar_url: account.avatar_url,
            };
            let res = api::Response::data(res);
            ok(HttpResponse::Ok().json(&res))
        })
        .map_err(move |err: KernelError| {
            slog_error!(logger, "{}", err);
            return err.error_response();
        })
        .from_err()
    );
}

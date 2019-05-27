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
    utils,
};
use futures::{
    future::{
        IntoFuture,
        ok,
        Future,
    },
};
use actix_web::{
    web, Error, HttpRequest, HttpResponse,
};
use rand::Rng;
use std::time::Duration;


pub fn put(password_data: web::Json<models::ResetPassowrdBody>, state: web::Data<api::State>, req: HttpRequest)
-> impl Future<Item = HttpResponse, Error = Error> {
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;
    let mut rng = rand::thread_rng();


    let session_id = match auth.session {
        Some(ref session) => Some(session.id),
        None => None,
    };

    // random sleep to prevent bruteforce and sidechannels attacks
    return tokio_timer::sleep(Duration::from_millis(rng.gen_range(250, 350))).into_future()
    .from_err()
    .and_then(move |_|
        state.db
        .send(controllers::ResetPassword{
            reset_password_id: password_data.id,
            token: password_data.token.clone(),
            new_password: password_data.new_password.clone(),
            request_id,
            session_id,
        }).flatten()
    )
    .from_err()
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
    .from_err();
}

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
};
use futures::{
    future::Future,
    future::ok,
    future::Either,
};
use actix_web::{
    web, Error, HttpRequest, HttpResponse, ResponseError,
};


pub fn post(session_id: web::Path<(uuid::Uuid)>, state: web::Data<api::State>, req: HttpRequest)
-> impl Future<Item = HttpResponse, Error = Error> {
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;

    if auth.session.is_none() || auth.account.is_none() {
        return Either::A(ok(KernelError::Unauthorized("Authentication required".to_string()).error_response()));
    }

    return Either::B(
        state.db
        .send(controllers::RevokeSession{
            actor: auth.account.unwrap(),
            session_id: session_id.into_inner(),
            request_id: request_id,
            current_session_id: auth.session.unwrap().id,
        })
        .map_err(|_| KernelError::ActixMailbox)
        .from_err()
        .and_then(move |res| {
            match res {
                Ok(_) => ok(HttpResponse::Ok().json(api::Response::data(models::NoData{}))),
                Err(err) => {
                    slog_error!(logger, "{}", err);
                    ok(err.error_response())
                },
            }
        })
    );
}

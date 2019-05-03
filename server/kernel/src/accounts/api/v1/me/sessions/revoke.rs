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
    future,
    future::Future,
};
use actix_web::{
    web, Error, HttpRequest, HttpResponse,
};


pub fn post(session_id: web::Path<(uuid::Uuid)>, state: web::Data<api::State>, req: HttpRequest)
-> impl Future<Item = HttpResponse, Error = Error> {
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;

    if auth.session.is_none() || auth.account.is_none() {
        return future::result(Ok(KernelError::Unauthorized("Authentication required".to_string()).error_response()))
            .responder();
    }

    // let session_id = match uuid::Uuid::parse_str(&session_id) {
    //     Ok(x) => x,
    //     Err(_) => return future::result(Ok(apiKernelError::Validation("session_id is not valid".to_string())).error_response()))
    //         .responder(),
    // };

    return state.db
    .send(controllers::RevokeSession{
        actor: auth.account.unwrap(),
        session_id: session_id.into_inner(),
        request_id: request_id,
        current_session_id: auth.session.unwrap().id,
    })
    .and_then(move |_| {
        let res = api::Response::data(models::NoData{});
        Ok(HttpResponse::Ok().json(&res))
    })
    .from_err() // MailboxError to KernelError
    .map_err(move |err: KernelError| {
        slog_error!(logger, "{}", err);
        return err;
    })
    .from_err()
    .responder();
}

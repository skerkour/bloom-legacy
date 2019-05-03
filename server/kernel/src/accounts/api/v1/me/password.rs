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


pub fn put(account_data: web::Json<models::UpdatePassword>, state: web::Data<api::State>, req: HttpRequest)
-> impl Future<Item = HttpResponse, Error = Error> {
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;
    let account_data = account_data.clone();

    if auth.session.is_none() || auth.account.is_none() {
        return future::result(Ok(KernelError::Unauthorized("Authentication required".to_string()).error_response()))
            .responder();
    }

    return state.db
    .send(controllers::UpdatePassword{
        current_session: auth.session.expect("unwraping auth session"),
        account: auth.account.expect("unwraping auth account"),
        current_password: account_data.current_password,
        new_password: account_data.new_password,
        request_id,
    })
    .from_err()
    .and_then(move |res|
        match res {
            Ok(_) => Ok(HttpResponse::Ok().json(api::Response::data(models::NoData{}))),
            Err(err) => Err(err),
        }
    )
    .from_err()
    .map_err(move |err: KernelError| {
        slog_error!(logger, "{}", err);
        return err;
    })
    .from_err()
    .responder();
}

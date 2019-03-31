use crate::{
    api,
    users::api::v1::models,
    log::macros::*,
    users::controllers,
    api::middlewares::{
        GetRequestLogger,
        GetRequestId,
        GetRequestAuth,
    },
    error::KernelError,
};
use futures::future::Future;
use actix_web::{
    FutureResponse, AsyncResponder, HttpResponse, HttpRequest, ResponseError, Json,
};
use futures::future;


pub fn put((user_data, req): (Json<models::UpdatePassword>, HttpRequest<api::State>))
-> FutureResponse<HttpResponse> {
    let state = req.state().clone();
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;
    let user_data = user_data.clone();

    if auth.session.is_none() || auth.user.is_none() {
        return future::result(Ok(KernelError::Unauthorized("Authentication required".to_string()).error_response()))
            .responder();
    }

    return state.db
    .send(controllers::UpdatePassword{
        current_session: auth.session.expect("unwraping auth session"),
        user: auth.user.expect("unwraping auth user"),
        current_password: user_data.current_password,
        new_password: user_data.new_password,
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

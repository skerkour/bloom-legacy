use futures::future::Future;
use actix_web::{
    FutureResponse, AsyncResponder, HttpResponse, HttpRequest, ResponseError,
};
use futures::future;
use kernel::{
    api,
    log::macros::*,
    api::middlewares::{
        GetRequestLogger,
        GetRequestAuth,
        GetRequestId,
    },
    KernelError,
};
use crate::{
    controllers,
};


pub fn delete(req: &HttpRequest<api::State>) -> FutureResponse<HttpResponse> {
    let state = req.state().clone();
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;

    if auth.session.is_none() || auth.account.is_none() {
        return future::result(Ok(KernelError::Unauthorized("Authentication required".to_string()).error_response()))
        .responder();
    }

    return state.db
    .send(controllers::ClearHistory{
        actor_id: auth.account.expect("unwraping non none account").id,
        session_id: auth.session.expect("unwraping non none session").id,
        request_id: request_id,
    })
    .from_err()
    .and_then(move |res| {
        match res {
            Ok(_) => {
                let res = api::Response::data(api::NoData{});
                Ok(HttpResponse::Ok().json(&res))
            },
            Err(err) => Err(err),
        }
    })
    .from_err()
    .map_err(move |err: KernelError| {
        slog_error!(logger, "{}", err);
        return err;
    })
    .from_err()
    .responder();
}

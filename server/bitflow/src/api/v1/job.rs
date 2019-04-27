use futures::{
    future::Future,
    future,
};
use actix_web::{
    FutureResponse, AsyncResponder, HttpResponse, HttpRequest, ResponseError,
};
use kernel::{
    api,
    log::macros::*,
    api::middlewares::{
        GetRequestLogger,
        GetRequestAuth,
        GetRequestId
    },
    KernelError,
};
use crate::{
    controllers,
    api::v1::models,
};


pub fn get(req: &HttpRequest<api::State>) -> FutureResponse<HttpResponse> {
    let state = req.state().clone();
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;

    if auth.service.is_none() || auth.service.unwrap() != api::middlewares::Service::Phaser {
        return future::result(Ok(KernelError::Unauthorized("Authentication required".to_string()).error_response()))
        .responder();
    }

    return state.db
    .send(controllers::StartDownload{
        // account_id: auth.account.expect("unwrapping non none account").id,
        // session_id: auth.session.expect("unwraping non none session").id,
        request_id: request_id,
    })
    .from_err()
    .and_then(move |res| {
        match res {
            Ok(download) => {
                let res = models::DownloadResponse::from(download);
                let res = api::Response::data(res);
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

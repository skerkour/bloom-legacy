use futures::future::Future;
use actix_web::{
    FutureResponse, AsyncResponder, HttpResponse, HttpRequest, ResponseError, Path,
};
use futures::future;
use kernel::{
    api,
    log::macros::*,
    api::middlewares::{
        GetRequestLogger,
        GetRequestAuth,
    },
    KernelError,
};
use crate::{
    api::v1::models,
    controllers,
};


pub fn get((download_id, req): (Path<(uuid::Uuid)>, HttpRequest<api::State>)) -> FutureResponse<HttpResponse> {
    let state = req.state().clone();
    let logger = req.logger();
    let auth = req.request_auth();

    if auth.session.is_none() || auth.account.is_none() {
        return future::result(Ok(KernelError::Unauthorized("Authentication required".to_string()).error_response()))
        .responder();
    }

    return state.db
    .send(controllers::FindDownload{
        download_id: download_id.into_inner(),
        account_id: auth.account.expect("unwrapping non none account").id,
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

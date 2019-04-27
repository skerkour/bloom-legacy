use futures::future::Future;
use actix_web::{
    FutureResponse, AsyncResponder, HttpResponse, HttpRequest, ResponseError, Json, Path,
};
use futures::future;
use kernel::{
    api,
    log::macros::*,
    api::middlewares::{
        GetRequestLogger,
        GetRequestId,
        GetRequestAuth,
    },
    KernelError,
};
use crate::{
    controllers,
    domain::download,
};


pub fn post((download_id, download_data, req): (Path<(uuid::Uuid)>, Json<download::CompleteData>, HttpRequest<api::State>)) -> FutureResponse<HttpResponse> {
    let state = req.state().clone();
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;

    if auth.service.is_none() || auth.service.unwrap() != api::middlewares::Service::Bitflow {
        return future::result(Ok(KernelError::Unauthorized("Authentication required".to_string()).error_response()))
        .responder();
    }

    return state.db
    .send(controllers::CompleteDownload{
        download_id: download_id.into_inner(),
        complete_data: download_data.clone(),
        s3_bucket: state.config.s3_bucket(),
        s3_client: state.s3_client.clone(),
        // actor_id: auth.account.expect("error unwraping non none account").id,
        // session_id: auth.session.expect("error unwraping non none session").id,
        request_id,
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

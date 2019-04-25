use futures::future::Future;
use actix_web::{
    FutureResponse, AsyncResponder, HttpResponse, HttpRequest, ResponseError, Json,
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
    api::v1::models,
    controllers,
};


pub fn post((upload_data, req): (Json<models::StartUploadSessionBody>, HttpRequest<api::State>)) -> FutureResponse<HttpResponse> {
    let state = req.state().clone();
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;

    if auth.session.is_none() || auth.account.is_none() {
        return future::result(Ok(KernelError::Unauthorized("Authentication required".to_string()).error_response()))
            .responder();
    }

    return state.db
    .send(controllers::StartUploadSession{
        file_name: upload_data.file_name.clone(),
        parent_id: upload_data.parent_id.clone(),
        s3_bucket: state.config.s3_bucket(),
        s3_region: state.config.aws_region(),
        account_id: auth.account.expect("error unwraping non none account").id,
        session_id: auth.session.expect("error unwraping non none session").id,
        request_id,
    })
    .from_err()
    .and_then(move |upload_session| {
        match upload_session {
            Ok(upload_session) => {
                let res = models::StartUploadSessionResponse{
                    id: upload_session.id,
                    presigned_url: upload_session.presigned_url,
                };
                let res = api::Response::data(res);
                Ok(HttpResponse::Created().json(&res))
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

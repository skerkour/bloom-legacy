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
    api::v1::models,
    controllers,
};


pub fn post((file_id, move_data, req): (Path<(uuid::Uuid)>, Json<models::MoveBody>, HttpRequest<api::State>)) -> FutureResponse<HttpResponse> {
    let state = req.state().clone();
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;

    if auth.session.is_none() || auth.account.is_none() {
        return future::result(Ok(KernelError::Unauthorized("Authentication required".to_string()).error_response()))
            .responder();
    }

    return state.db
    .send(controllers::Move{
        to: move_data.to.clone(),
        file_id: file_id.clone(),
        owner_id: auth.account.expect("error unwraping non none account").id,
        session_id: auth.session.expect("error unwraping non none session").id,
        request_id,
    })
    .from_err()
    .and_then(move |moved_file| {
        match moved_file {
            Ok(moved_file) => {
                let res = models::FileBody{
                    id: moved_file.id,
                    created_at: moved_file.created_at,
                    updated_at: moved_file.updated_at,
                    name: moved_file.name,
                    type_: moved_file.type_,
                    size: moved_file.size,
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

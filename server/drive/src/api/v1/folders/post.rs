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


pub fn post((create_data, req): (Json<models::CreateFolderBody>, HttpRequest<api::State>)) -> FutureResponse<HttpResponse> {
    let state = req.state().clone();
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;

    if auth.session.is_none() || auth.account.is_none() {
        return future::result(Ok(KernelError::Unauthorized("Authentication required".to_string()).error_response()))
            .responder();
    }

    return state.db
    .send(controllers::CreateFolder{
        parent_id: create_data.parent_id.clone(),
        name: create_data.name.clone(),
        owner_id: auth.account.expect("error unwraping non none account").id,
        session_id: auth.session.expect("error unwraping non none session").id,
        request_id,
    })
    .from_err()
    .and_then(move |new_folder| {
        match new_folder {
            Ok(new_folder) => {
                let res = models::FileBody{
                    id: new_folder.id,
                    created_at: new_folder.created_at,
                    updated_at: new_folder.updated_at,
                    name: new_folder.name,
                    type_: new_folder.type_,
                    size: new_folder.size,
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

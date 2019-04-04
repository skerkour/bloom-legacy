use futures::future::Future;
use serde::{Deserialize};
use actix_web::{
    FutureResponse, AsyncResponder, HttpResponse, HttpRequest, ResponseError, Query,
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
    controllers,
    api::v1::models,
    domain::{
        File,
    }
};

#[derive(Deserialize)]
pub struct QueryParams {
   id: Option<uuid::Uuid>,
}


pub fn get((query_params, req): (Query<QueryParams>, HttpRequest<api::State>)) -> FutureResponse<HttpResponse> {
    let state = req.state().clone();
    let logger = req.logger();
    let auth = req.request_auth();

    if auth.session.is_none() || auth.account.is_none() {
        return future::result(Ok(KernelError::Unauthorized("Authentication required".to_string()).error_response()))
        .responder();
    }

    return state.db
    .send(controllers::FindFolder{
        id: query_params.id.clone(),
        owner_id:  auth.account.expect("unwrapping non none account").id,
    })
    .from_err()
    .and_then(move |res| {
        match res {
            Ok((path, folder, children)) => {
                let folder = models::FolderBody{
                    id: folder.id,
                    created_at: folder.created_at,
                    updated_at: folder.updated_at,
                    path: vec!(path),
                    name: folder.name,
                    type_: folder.type_,
                    size: folder.size,
                    children: Vec::new(),
                };
                let res = api::Response::data(folder);
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

use kernel::{
    api,
    log::macros::*,
    api::middlewares::{
        GetRequestLogger,
        GetRequestAuth,
    },
    KernelError,
};
use futures::{
    future::Future,
    future::ok,
    future::Either,
};
use actix_web::{
    web, Error, HttpRequest, HttpResponse, ResponseError,
};
use serde::{Deserialize};
use crate::{
    controllers,
    api::v1::models,
};


#[derive(Deserialize)]
pub struct QueryParams {
   id: Option<uuid::Uuid>,
}

pub fn get(query_params: web::Query<QueryParams>, state: web::Data<api::State>, req: HttpRequest)
-> impl Future<Item = HttpResponse, Error = Error> {
    let logger = req.logger();
    let auth = req.request_auth();

    if auth.session.is_none() || auth.account.is_none() {
        return Either::A(ok(KernelError::Unauthorized("Authentication required".to_string()).error_response()));
    }

    return Either::B(
        state.db
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
                        path,
                        name: folder.name,
                        type_: folder.type_,
                        size: folder.size,
                        files: children.into_iter().map(|file|
                            models::FileBody{
                                id: file.id,
                                created_at: file.created_at,
                                updated_at: file.updated_at,
                                name: file.name,
                                type_: file.type_,
                                size: file.size,
                            }
                        ).collect(),
                    };
                    let res = api::Response::data(folder);
                    ok(HttpResponse::Ok().json(&res))
                },
                Err(err) => {
                    slog_error!(logger, "{}", err);
                    ok(err.error_response())
                },
            }
        })
    );
}

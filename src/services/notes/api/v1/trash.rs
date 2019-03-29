use crate::{
    api,
    log::macros::*,
    services::notes::controllers,
    services::notes::api::v1::models,
    api::middlewares::{
        GetRequestLogger,
        GetRequestId,
        GetRequestAuth,
    },
    error::KernelError,
};
use futures::future::Future;
use actix_web::{
    FutureResponse, AsyncResponder, HttpResponse, HttpRequest, ResponseError,
};
use futures::future;


pub fn get(req: &HttpRequest<api::State>) -> FutureResponse<HttpResponse> {
    let state = req.state().clone();
    let logger = req.logger();
    let auth = req.request_auth();

    if auth.session.is_none() || auth.account.is_none() {
        return future::result(Ok(KernelError::Unauthorized("Authentication required".to_string()).error_response()))
        .responder();
    }

    return state.db
    .send(controllers::FindUserTrash{
        account_id: auth.account.expect("unwrapping non none account").id,
    })
    .from_err()
    .and_then(move |notes| {
        match notes {
            Ok(notes) => {
                let notes: Vec<models::NoteResponse> = notes.into_iter().map(|note| {
                    models::NoteResponse{
                        id: note.id,
                        created_at: note.created_at,
                        updated_at: note.updated_at,
                        archived_at: note.archived_at,
                        removed_at: note.removed_at,
                        title: note.title,
                        body: note.body,
                    }
                }).collect();
                let res = api::Response::data(notes);
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

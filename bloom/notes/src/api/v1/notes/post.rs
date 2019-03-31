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
    controllers,
    api::v1::models,
};


pub fn post((note_data, req): (Json<models::CreateNoteBody>, HttpRequest<api::State>)) -> FutureResponse<HttpResponse> {
    let state = req.state().clone();
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;

    if auth.session.is_none() || auth.user.is_none() {
        return future::result(Ok(KernelError::Unauthorized("Authentication required".to_string()).error_response()))
            .responder();
    }

    return state.db
    .send(controllers::CreateNote{
        title: note_data.title.clone(),
        body: note_data.body.clone(),
        user_id: auth.user.expect("error unwraping non none user").id,
        session_id: auth.session.expect("error unwraping non none session").id,
        request_id,
    })
    .from_err()
    .and_then(move |note| {
        match note {
            Ok(note) => {
                let res = models::NoteResponse{
                    id: note.id,
                    created_at: note.created_at,
                    updated_at: note.updated_at,
                    archived_at: note.archived_at,
                    removed_at: note.removed_at,
                    title: note.title,
                    body: note.body,
                };
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

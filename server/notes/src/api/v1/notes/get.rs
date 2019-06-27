use crate::{api::v1::models, controllers};
use actix_web::{web, Error, HttpRequest, HttpResponse, ResponseError};
use futures::{future::ok, future::Either, future::Future};
use kernel::{
    api,
    api::middlewares::{GetRequestAuth, GetRequestLogger},
    log::macros::*,
    KernelError,
};

pub fn get(
    state: web::Data<api::State>,
    req: HttpRequest,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let logger = req.logger();
    let auth = req.request_auth();

    if auth.session.is_none() || auth.account.is_none() {
        return Either::A(ok(KernelError::Unauthorized(
            "Authentication required".to_string(),
        )
        .error_response()));
    }

    return Either::B(
        state
            .db
            .send(controllers::FindAccountNotes {
                account_id: auth.account.expect("unwrapping non none account").id,
            })
            .map_err(|_| KernelError::ActixMailbox)
            .from_err()
            .and_then(move |notes| match notes {
                Ok(notes) => {
                    let notes: Vec<models::NoteResponse> = notes
                        .into_iter()
                        .map(|note| models::NoteResponse {
                            id: note.id,
                            created_at: note.created_at,
                            updated_at: note.updated_at,
                            archived_at: note.archived_at,
                            removed_at: note.removed_at,
                            title: note.title,
                            body: note.body,
                        })
                        .collect();
                    let res = api::Response::data(notes);
                    Ok(HttpResponse::Ok().json(&res))
                }
                Err(err) => {
                    slog_error!(logger, "{}", err);
                    Err(err.into())
                }
            }),
    );
}

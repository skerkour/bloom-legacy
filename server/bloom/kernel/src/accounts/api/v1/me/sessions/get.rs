use crate::{
    api,
    api::middlewares::{GetRequestAuth, GetRequestLogger},
    error::KernelError,
    log::macros::*,
    accounts::api::v1::models,
    accounts::controllers,
};
use actix_web::{web, Error, HttpRequest, HttpResponse, ResponseError};
use futures::{future::ok, future::Either, future::Future};

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
            .send(controllers::FindSessionsForAccount {
                account_id: auth.account.expect("unwrapping non none account").id,
            })
            .map_err(|_| KernelError::ActixMailbox)
            .from_err()
            .and_then(move |sessions| match sessions {
                Ok(sessions) => {
                    let sessions: Vec<models::Session> = sessions
                        .into_iter()
                        .map(|session| models::Session {
                            id: session.id,
                            created_at: session.created_at,
                            ip: session.ip,
                            location: session.location,
                            device: session.device,
                        })
                        .collect();
                    let res = api::Response::data(sessions);
                    ok(HttpResponse::Ok().json(&res))
                }
                Err(err) => {
                    slog_error!(logger, "{}", err);
                    ok(err.error_response())
                }
            }),
    );
}

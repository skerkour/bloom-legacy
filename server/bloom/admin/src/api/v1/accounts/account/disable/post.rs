use crate::controllers;
use actix_web::{web, Error, HttpRequest, HttpResponse, ResponseError};
use futures::{future::ok, future::Either, future::Future};
use kernel::{
    api,
    api::middlewares::{GetRequestAuth, GetRequestId, GetRequestLogger},
    error::KernelError,
    log::macros::*,
};

pub fn post(
    account_id: web::Path<(uuid::Uuid)>,
    state: web::Data<api::State>,
    req: HttpRequest,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;

    if auth.session.is_none() || auth.account.is_none() {
        return Either::A(ok(KernelError::Unauthorized(
            "Authentication required".to_string(),
        )
        .error_response()));
    }

    return Either::B(
        state
            .db
            .send(controllers::DisableAccount {
                actor: auth.account.expect("unwraping non none account"),
                account_id: account_id.into_inner(),
                request_id,
                session_id: auth.session.expect("unwraping non none session").id,
            })
            .map_err(|_| KernelError::ActixMailbox)
            .from_err()
            .and_then(move |res: Result<(), KernelError>| match res {
                Ok(_) => {
                    let res = api::Response::data(api::NoData {});
                    ok(HttpResponse::Ok().json(&res))
                }
                Err(err) => {
                    slog_error!(logger, "{}", err);
                    ok(err.error_response())
                }
            }),
    );
}

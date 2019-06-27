use crate::{
    api,
    api::middlewares::{GetRequestAuth, GetRequestId, GetRequestLogger},
    error::KernelError,
    log::macros::*,
    myaccount::controllers,
};
use actix_web::{http::StatusCode, web, Error, HttpRequest, HttpResponse, ResponseError};
use futures::{future::ok, future::Either, future::Future};

pub fn delete(
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
            .send(controllers::DeleteAccount {
                account: auth.account.expect("unwraping non none account"),
                request_id,
                session_id: auth.session.expect("unwraping non none session").id,
            })
            .map_err(|_| KernelError::ActixMailbox)
            .from_err()
            .and_then(move |res: Result<(), KernelError>| match res {
                Ok(_) => ok(HttpResponse::Ok().status(StatusCode::ACCEPTED).finish()),
                Err(err) => {
                    slog_error!(logger, "{}", err);
                    ok(err.error_response())
                }
            }),
    );
}

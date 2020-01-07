use crate::{api::v1::models, controllers};
use actix_web::{web, Error, HttpRequest, HttpResponse, ResponseError};
use futures::{future::ok, future::Either, future::Future};
use kernel::{
    api,
    api::middlewares::{GetRequestAuth, GetRequestId, GetRequestLogger},
    log::macros::*,
    KernelError,
};

pub fn get(
    state: web::Data<api::State>,
    req: HttpRequest,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;

    if auth.service.is_none() || auth.service.unwrap() != api::middlewares::Service::Phaser {
        return Either::A(ok(KernelError::Unauthorized(
            "Authentication required".to_string(),
        )
        .error_response()));
    }

    return Either::B(
        state
            .db
            .send(controllers::StartScan {
                // account_id: auth.account.expect("unwrapping non none account").id,
                // session_id: auth.session.expect("unwraping non none session").id,
                request_id,
            })
            .map_err(|_| KernelError::ActixMailbox)
            .from_err()
            .and_then(move |res| match res {
                Ok(report) => {
                    let res = models::ReportJob::from(report);
                    let res = api::Response::data(res);
                    ok(HttpResponse::Ok().json(&res))
                }
                Err(err) => {
                    slog_error!(logger, "{}", err);
                    ok(err.error_response())
                }
            }),
    );
}

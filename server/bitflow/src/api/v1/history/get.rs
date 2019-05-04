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
    future::{
        IntoFuture,
        Either,
        ok,
        Future,
    },
};
use actix_web::{
    web, Error, HttpRequest, HttpResponse, ResponseError,
};
use crate::{
    controllers,
    api::v1::models,
};


pub fn get(state: web::Data<api::State>, req: HttpRequest) -> impl Future<Item = HttpResponse, Error = Error> {
    let logger = req.logger();
    let auth = req.request_auth();

    if auth.session.is_none() || auth.account.is_none() {
        return Either::A(ok(KernelError::Unauthorized("Authentication required".to_string()).error_response()));
    }

    return Either::B(
        state.db
        .send(controllers::FindHistory{
            account_id: auth.account.expect("unwrapping non none account").id,
        })
        .from_err()
        .and_then(move |downloads| {
            match downloads {
                Ok(downloads) => {
                    let downloads: Vec<models::DownloadResponse> = downloads.into_iter().map(From::from).collect();
                    let res = api::Response::data(downloads);
                    Ok(HttpResponse::Ok().json(&res))
                },
                Err(err) => {
                    slog_error!(logger, "{}", err);
                    ok(err.error_response())
                },
            }
        })
    );
}

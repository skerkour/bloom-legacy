use crate::{
    api,
    api::middlewares::GetRequestLogger,
    error::KernelError,
    log::macros::*,
    accounts::{api::v1::models, controllers},
};
use actix_web::{web, Error, HttpRequest, HttpResponse, ResponseError};
use futures::{future::ok, future::Future};

pub fn get(
    username: web::Path<(String)>,
    state: web::Data<api::State>,
    req: HttpRequest,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let logger = req.logger();

    return state
        .db
        .send(controllers::FindUser {
            username: username.into_inner(),
        })
        .map_err(|_| KernelError::ActixMailbox)
        .from_err()
        .and_then(move |res: Result<_, KernelError>| match res {
            Ok(account) => {
                let account_res: models::PublicProfileResponse = account.into();
                let res = api::Response::data(account_res);
                ok(HttpResponse::Ok().json(&res))
            }
            Err(err) => {
                slog_error!(logger, "{}", err);
                ok(err.error_response())
            }
        });
}

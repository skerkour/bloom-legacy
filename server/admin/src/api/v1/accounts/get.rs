use crate::{api::v1::models, controllers};
use actix_web::{web, Error, HttpRequest, HttpResponse, ResponseError};
use futures::{future::ok, future::Either, future::Future};
use kernel::{
    api,
    api::middlewares::{GetRequestAuth, GetRequestLogger},
    log::macros::*,
    KernelError,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryParams {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub username: Option<String>,
    pub email: Option<String>,
}

pub fn get(
    query_params: web::Query<QueryParams>,
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
            .send(controllers::FindAccounts {
                actor: auth.account.expect("unwraping non none account"),
                offset: query_params.offset,
                limit: query_params.limit,
                email: query_params.email.clone(),
                username: query_params.username.clone(),
            })
            .map_err(|_| KernelError::ActixMailbox)
            .from_err()
            .and_then(move |res| match res {
                Ok((accounts, total)) => {
                    let accounts: Vec<models::AccountResponse> =
                        accounts.into_iter().map(From::from).collect();
                    let res = models::AccountsResponse {
                        hits: accounts,
                        total: total as u64,
                    };
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

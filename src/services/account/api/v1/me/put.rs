use crate::{
    api,
    services::account::api::v1::models,
    log::macros::*,
    services::account::controllers,
    api::middlewares::{
        GetRequestLogger,
        GetRequestId,
        GetRequestAuth,
    },
    error::KernelError,
    services::common::utils,
};
use futures::future::Future;
use actix_web::{
    FutureResponse, AsyncResponder, HttpResponse, HttpRequest, ResponseError, Json,
};
use futures::future;


pub fn put((account_data, req): (Json<models::UpdateAccount>, HttpRequest<api::State>))
-> FutureResponse<HttpResponse> {
    let state = req.state().clone();
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;
    let account_data = account_data.clone();

    if auth.session.is_none() || auth.account.is_none() {
        return future::result(Ok(KernelError::Unauthorized("Authentication required".to_string()).error_response()))
            .responder();
    }

    return state.db
    .send(controllers::UpdateAccount{
        account: auth.account.expect("unwraping non none account"),
        avatar_url: account_data.avatar_url,
        first_name: account_data.first_name,
        last_name: account_data.last_name,
        request_id,
        session_id: auth.session.expect("unwraping non none session").id,
    })
    .from_err()
    .and_then(move |account| {
        match account {
            Ok(account) => {
                let res = models::MeResponse{
                    id: account.id,
                    created_at: account.created_at,
                    first_name: account.first_name,
                    last_name: account.last_name,
                    username: account.username,
                    email: account.email,
                    avatar_url: account.avatar_url,
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

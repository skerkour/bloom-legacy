use crate::{
    api,
    myaccount::api::v1::models,
    log::macros::*,
    myaccount::controllers,
    api::middlewares::{
        GetRequestLogger,
        GetRequestId,
        GetRequestAuth,
    },
    error::KernelError,
};
use futures::{
    future::Future,
    future::ok,
    future::Either,
};
use actix_web::{
    web, Error, HttpRequest, HttpResponse, ResponseError,
};


pub fn put(account_data: web::Json<models::UpdateAccount>, state: web::Data<api::State>, req: HttpRequest)
-> impl Future<Item = HttpResponse, Error = Error> {
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;
    let account_data = account_data.clone();

    if auth.session.is_none() || auth.account.is_none() {
        return Either::A(ok(KernelError::Unauthorized("Authentication required".to_string()).error_response()));
    }

    return Either::B(
        state.db
        .send(controllers::UpdateAccount{
            account: auth.account.expect("unwraping non none account"),
            avatar_url: account_data.avatar_url,
            first_name: account_data.first_name,
            last_name: account_data.last_name,
            request_id,
            session_id: auth.session.expect("unwraping non none session").id,
        })
        .map_err(|_| KernelError::ActixMailbox)
        .from_err()
        .and_then(move |account: Result<_, KernelError>| {
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
                    ok(HttpResponse::Ok().json(&res))
                },
                Err(err) => {
                    slog_error!(logger, "{}", err);
                    ok(err.error_response())
                },
            }
        })
    );
}

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


pub fn put(account_data: web::Json<models::UpdatePassword>, state: web::Data<api::State>, req: HttpRequest)
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
        .send(controllers::UpdatePassword{
            current_session: auth.session.expect("unwraping auth session"),
            account: auth.account.expect("unwraping auth account"),
            current_password: account_data.current_password,
            new_password: account_data.new_password,
            request_id,
        })
        .map_err(|_| KernelError::ActixMailbox)
        .from_err()
        .and_then(move |res|
            match res {
                Ok(_) => ok(HttpResponse::Ok().json(api::Response::data(api::NoData{}))),
                Err(err) => {
                    slog_error!(logger, "{}", err);
                    ok(err.error_response())
                },
            }
        )
    );
}

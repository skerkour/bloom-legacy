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
    future::Future,
    future::ok,
    future::Either,
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
        .send(controllers::FindTrashed{
            owner_id:  auth.account.expect("unwrapping non none account").id,
        })
        .map_err(|_| KernelError::ActixMailbox)
        .from_err()
        .and_then(move |trash| {
            match trash {
                Ok(trash) => {
                    let res: Vec<models::FileBody> = trash.into_iter().map(|file|
                        models::FileBody{
                            id: file.id,
                            created_at: file.created_at,
                            updated_at: file.updated_at,
                            name: file.name,
                            type_: file.type_,
                            size: file.size,
                        }
                    ).collect();
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

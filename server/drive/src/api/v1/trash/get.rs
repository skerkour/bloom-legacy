use futures::future::Future;
use actix_web::{
    FutureResponse, AsyncResponder, HttpResponse, HttpRequest, ResponseError,
};
use futures::future;
use kernel::{
    api,
    log::macros::*,
    api::middlewares::{
        GetRequestLogger,
        GetRequestAuth,
    },
    KernelError,
};
use crate::{
    controllers,
    api::v1::models,
};


pub fn get(req: &HttpRequest<api::State>) -> FutureResponse<HttpResponse> {
    let state = req.state().clone();
    let logger = req.logger();
    let auth = req.request_auth();

    if auth.session.is_none() || auth.account.is_none() {
        return future::result(Ok(KernelError::Unauthorized("Authentication required".to_string()).error_response()))
        .responder();
    }

    return state.db
    .send(controllers::FindTrashed{
        owner_id:  auth.account.expect("unwrapping non none account").id,
    })
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

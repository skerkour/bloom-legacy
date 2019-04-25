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
    .send(controllers::FindAlbums{
        account_id: auth.account.expect("unwrapping non none account").id,
    })
    .from_err()
    .and_then(move |albums| {
        match albums {
            Ok(albums) => {
                let albums: Vec<models::AlbumResponse> = albums.into_iter().map(From::from).collect();
                let res = api::Response::data(albums);
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

use crate::{
    api,
    users::api::v1::models,
    log::macros::*,
    users::controllers,
    api::middlewares::{
        GetRequestLogger,
        GetRequestId,
        GetRequestAuth,
    },
    error::KernelError,
};
use futures::future::Future;
use actix_web::{
    FutureResponse, AsyncResponder, HttpResponse, HttpRequest, ResponseError, Json,
};
use futures::future;


pub fn put((user_data, req): (Json<models::UpdateUser>, HttpRequest<api::State>))
-> FutureResponse<HttpResponse> {
    let state = req.state().clone();
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;
    let user_data = user_data.clone();

    if auth.session.is_none() || auth.user.is_none() {
        return future::result(Ok(KernelError::Unauthorized("Authentication required".to_string()).error_response()))
            .responder();
    }

    return state.db
    .send(controllers::UpdateUser{
        user: auth.user.expect("unwraping non none user"),
        avatar_url: user_data.avatar_url,
        first_name: user_data.first_name,
        last_name: user_data.last_name,
        request_id,
        session_id: auth.session.expect("unwraping non none session").id,
    })
    .from_err()
    .and_then(move |user| {
        match user {
            Ok(user) => {
                let res = models::MeResponse{
                    id: user.id,
                    created_at: user.created_at,
                    first_name: user.first_name,
                    last_name: user.last_name,
                    username: user.username,
                    email: user.email,
                    avatar_url: user.avatar_url,
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

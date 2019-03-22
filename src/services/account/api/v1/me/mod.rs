mod logs;
mod sessions;


use crate::{
    api,
    services::account::api::v1::models,
    api::middlewares::{
        GetRequestAuth,
    },
    error::KernelError,
};
use actix_web::{
    HttpResponse, HttpRequest,
    error::ResponseError,
};


pub fn me_get(req: &HttpRequest<api::State>) -> HttpResponse {
    let auth = req.request_auth();

    if auth.session.is_none() || auth.account.is_none() {
        return api::Error::from(KernelError::Unauthorized("Authentication required".to_string())).error_response();
    }

    let account = auth.account.unwrap();
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
    return HttpResponse::Ok().json(&res);
}

use crate::{
    api, api::middlewares::GetRequestAuth, error::KernelError, myaccount::api::v1::models,
};
use actix_web::{error::ResponseError, HttpRequest, HttpResponse};

pub fn get(req: HttpRequest) -> HttpResponse {
    let auth = req.request_auth();

    if auth.session.is_none() || auth.account.is_none() {
        return KernelError::Unauthorized("Authentication required".to_string()).error_response();
    }

    let account = auth.account.unwrap();
    let res = models::MeResponse {
        id: account.id,
        created_at: account.created_at,
        first_name: account.first_name,
        last_name: account.last_name,
        username: account.username,
        email: account.email,
        avatar_url: account.avatar_url,
        is_admin: account.is_admin,
    };
    let res = api::Response::data(res);
    return HttpResponse::Ok().json(&res);
}

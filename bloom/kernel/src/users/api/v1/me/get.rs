use crate::{
    api,
    users::api::v1::models,
    api::middlewares::{
        GetRequestAuth,
    },
    error::KernelError,
};
use actix_web::{
    HttpResponse, HttpRequest,
    error::ResponseError,
};


pub fn get(req: &HttpRequest<api::State>) -> HttpResponse {
    let auth = req.request_auth();

    if auth.session.is_none() || auth.user.is_none() {
        return KernelError::Unauthorized("Authentication required".to_string()).error_response();
    }

    let user = auth.user.unwrap();
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
    return HttpResponse::Ok().json(&res);
}

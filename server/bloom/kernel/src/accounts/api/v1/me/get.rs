use crate::{
    api, api::middlewares::GetRequestAuth, error::KernelError, accounts::api::v1::models,
};
use actix_web::{error::ResponseError, HttpRequest, HttpResponse};

pub fn get(req: HttpRequest) -> HttpResponse {
    let auth = req.request_auth();

    if auth.session.is_none() || auth.account.is_none() {
        return KernelError::Unauthorized("Authentication required".to_string()).error_response();
    }

    let account = auth.account.unwrap();
    let res: models::MeResponse = account.into();
    let res = api::Response::data(res);
    return HttpResponse::Ok().json(&res);
}

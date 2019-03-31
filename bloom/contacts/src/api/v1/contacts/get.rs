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
    api::v1::models,
    controllers,
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
    .send(controllers::FindContactsForUser{
        account_id: auth.account.expect("unwrapping non none account").id,
    })
    .from_err()
    .and_then(move |contacts| {
        match contacts {
            Ok(contacts) => {
                let contacts: Vec<models::ContactResponse> = contacts.into_iter().map(|contact| {
                    models::ContactResponse{
                        id: contact.id,
                        created_at: contact.created_at,
                        updated_at: contact.updated_at,
                        addresses: contact.addresses,
                        birthday: contact.birthday,
                        company: contact.company,
                        emails: contact.emails,
                        first_name: contact.first_name,
                        last_name: contact.last_name,
                        notes: contact.notes,
                        occupation: contact.occupation,
                        organizations: contact.organizations,
                        phones: contact.phones,
                        websites: contact.websites,
                    }
                }).collect();
                let res = api::Response::data(contacts);
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

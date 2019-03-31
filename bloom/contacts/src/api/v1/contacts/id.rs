use futures::future::Future;
use actix_web::{
    FutureResponse, AsyncResponder, HttpResponse, HttpRequest, ResponseError, Path,
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


pub fn get((contact_id, req): (Path<(uuid::Uuid)>, HttpRequest<api::State>)) -> FutureResponse<HttpResponse> {
    let state = req.state().clone();
    let logger = req.logger();
    let auth = req.request_auth();

    if auth.session.is_none() || auth.account.is_none() {
        return future::result(Ok(KernelError::Unauthorized("Authentication required".to_string()).error_response()))
        .responder();
    }

    return state.db
    .send(controllers::FindContactForAccount{
        contact_id: contact_id.into_inner(),
        account_id: auth.account.expect("unwrapping non none account").id,
    })
    .from_err()
    .and_then(move |contact| {
        match contact {
            Ok(contact) => {
                let contact = models::ContactResponse{
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
                };
                let res = api::Response::data(contact);
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

use crate::{api::v1::models, controllers};
use actix_web::{web, Error, HttpRequest, HttpResponse, ResponseError};
use futures::future::{ok, Either, Future};
use kernel::{
    api,
    api::middlewares::{GetRequestAuth, GetRequestLogger},
    log::macros::*,
    KernelError,
};

pub fn get(
    state: web::Data<api::State>,
    req: HttpRequest,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let logger = req.logger();
    let auth = req.request_auth();

    if auth.session.is_none() || auth.account.is_none() {
        return Either::A(ok(KernelError::Unauthorized(
            "Authentication required".to_string(),
        )
        .error_response()));
    }

    return Either::B(
        state
            .db
            .send(controllers::FindContactsForAccount {
                account_id: auth.account.expect("unwrapping non none account").id,
            })
            .map_err(|_| KernelError::ActixMailbox)
            .from_err()
            .and_then(move |contacts| match contacts {
                Ok(contacts) => {
                    let contacts: Vec<models::ContactResponse> = contacts
                        .into_iter()
                        .map(|contact| models::ContactResponse {
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
                        })
                        .collect();
                    let res = api::Response::data(contacts);
                    ok(HttpResponse::Ok().json(&res))
                }
                Err(err) => {
                    slog_error!(logger, "{}", err);
                    ok(err.error_response())
                }
            }),
    );
}

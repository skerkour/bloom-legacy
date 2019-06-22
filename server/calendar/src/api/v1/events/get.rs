use kernel::{
    api,
    log::macros::*,
    api::middlewares::{
        GetRequestLogger,
        GetRequestAuth,
    },
    KernelError,
};
use chrono::Datelike;
use serde::{Deserialize};
use futures::{
    future::Future,
    future::ok,
    future::Either,
};
use actix_web::{
    web, Error, HttpRequest, HttpResponse, ResponseError,
};
use crate::{
    validators,
    controllers,
    api::v1::models,
};

#[derive(Deserialize)]
pub struct QueryParams {
    pub start_at: Option<chrono::DateTime<chrono::Utc>>,
    pub end_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub fn get(query_params: web::Query<QueryParams>, state: web::Data<api::State>, req: HttpRequest) -> impl Future<Item = HttpResponse, Error = Error> {
    let logger = req.logger();
    let auth = req.request_auth();

    if auth.session.is_none() || auth.account.is_none() {
        return Either::A(ok(KernelError::Unauthorized("Authentication required".to_string()).error_response()));
    }

    let now = chrono::Utc::now();
    let start_at = query_params.start_at.unwrap_or_else(|| now.with_day(1).unwrap());
    let end_at = query_params.end_at.unwrap_or_else(|| start_at + chrono::Duration::days(31));

    if let Err(err) = validators::date(start_at, end_at) {
        return Either::A(ok(err.error_response()));
    }

    Either::B(
        state.db
        .send(controllers::FindEvents{
            owner_id:  auth.account.expect("unwrapping non none account").id,
            start_at,
            end_at,
        })
        .map_err(|_| KernelError::ActixMailbox)
        .from_err()
        .and_then(move |events_resp| {
            match events_resp {
                Ok(events) => {
                    let res: Vec<models::EventBody> = events.into_iter().map(|event|
                        models::EventBody{
                            id: event.id,
                            created_at: event.created_at,
                            updated_at: event.updated_at,
                            title: event.title,
                            description: event.description,
                            start_at: event.start_at,
                            end_at: event.end_at,
                        }
                    ).collect();
                    let res = api::Response::data(res);
                    ok(HttpResponse::Ok().json(&res))
                },
                Err(err) => {
                    slog_error!(logger, "{}", err);
                    ok(err.error_response())
                },
            }
        })
    )
}

use crate::{
    api,
    api::middlewares::{GetRequestAuth, GetRequestId, GetRequestLogger},
    error::KernelError,
    log::macros::*,
    accounts,
    accounts::api::v1::models,
    accounts::controllers,
};
use actix_multipart::{Field, Multipart, MultipartError};
use actix_web::{error, web, Error, HttpRequest, HttpResponse, ResponseError};
use futures::{future, future::ok, future::Either, Future, IntoFuture, Stream};

pub fn put(
    multipart: Multipart,
    state: web::Data<api::State>,
    req: HttpRequest,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;

    if auth.session.is_none() || auth.account.is_none() {
        return Either::A(ok(KernelError::Unauthorized(
            "Authentication required".to_string(),
        )
        .error_response()));
    }

    return Either::B(
        multipart
            .map_err(|err| KernelError::Internal(err.to_string()))
            .map(handle_multipart_item)
            .flatten()
            .collect()
            .into_future()
            .map_err(|_| KernelError::Validation("file too large".to_string()))
            .and_then(move |avatar| {
                state
                    .db
                    .send(controllers::UpdateAvatar {
                        account: auth.account.expect("unwrapping non none account"),
                        avatar: avatar.get(0).expect("processing file").to_vec(),
                        s3_bucket: state.config.s3.bucket.clone(),
                        s3_base_url: state.config.s3.base_url.clone(),
                        s3_client: state.s3_client.clone(),
                        request_id,
                        session_id: auth.session.expect("unwraping non none session").id,
                    })
                    .flatten()
            })
            .and_then(move |account| {
                let res: models::MeResponse = account.into();
                let res = api::Response::data(res);
                Ok(HttpResponse::Ok().json(&res))
            })
            .from_err()
            .map_err(move |err: KernelError| {
                slog_error!(logger, "{}", err);
                return err.into();
            }),
    );
}

fn handle_multipart_item(field: Field) -> Box<dyn Stream<Item = Vec<u8>, Error = Error>> {
    return Box::new(read_file(field).into_stream());
}

fn read_file(field: Field) -> Box<dyn Future<Item = Vec<u8>, Error = Error>> {
    Box::new(
        field
            .fold(
                Vec::new(),
                |mut acc, bytes| -> future::FutureResult<_, MultipartError> {
                    acc.extend_from_slice(&bytes);
                    if acc.len() > accounts::AVATAR_MAX_SIZE {
                        return future::err(MultipartError::Payload(error::PayloadError::Overflow));
                    }
                    future::ok(acc)
                },
            )
            .map_err(error::ErrorInternalServerError),
    )
}

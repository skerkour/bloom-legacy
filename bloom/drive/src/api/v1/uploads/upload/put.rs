use crate::{api::v1::models, controllers};
use actix_multipart::{Field, Multipart, MultipartError};
use actix_web::{error, web, Error, HttpRequest, HttpResponse, ResponseError};
use futures::{future, future::ok, future::Either, Future, IntoFuture, Stream};
use kernel::{
    api,
    api::middlewares::{GetRequestAuth, GetRequestId, GetRequestLogger},
    log::macros::*,
    KernelError,
};
use std::fs;
use std::io::Write;

pub fn put(
    upload_id: web::Path<(uuid::Uuid)>,
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

    let upload_dir = format!("tmp/drive/uploads/{}_{}", request_id, upload_id.clone());
    match fs::create_dir_all(&upload_dir) {
        Ok(_) => {}
        Err(err) => return Either::A(ok(KernelError::from(err).error_response())),
    }
    let upload_file_path = format!("{}/upload", &upload_dir);
    let upload_file_path2 = upload_file_path.clone();

    return Either::B(
        multipart
            .map_err(|err| KernelError::Internal(err.to_string()))
            .map(move |field| handle_upload(&upload_file_path, field))
            .flatten()
            .collect()
            .into_future()
            .map_err(|_| KernelError::Internal("Error uploading file".to_string()))
            .and_then(move |_| {
                state
                    .db
                    .send(controllers::CompleteUpload {
                        upload_id: upload_id.into_inner(),
                        s3_bucket: state.config.s3.bucket.clone(),
                        s3_client: state.s3_client.clone(),
                        file_path: upload_file_path2,
                        directory: upload_dir,
                        account_id: auth.account.expect("error unwraping non none account").id,
                        session_id: auth.session.expect("error unwraping non none session").id,
                        request_id,
                    })
                    .flatten()
            })
            .and_then(move |file| {
                let file = models::FileBody {
                    id: file.id,
                    created_at: file.created_at,
                    updated_at: file.updated_at,
                    name: file.name,
                    type_: file.type_,
                    size: file.size,
                };
                let res = api::Response::data(file);
                Ok(HttpResponse::Created().json(&res))
            })
            .from_err()
            .map_err(move |err: KernelError| {
                slog_error!(logger, "{}", err);
                return err.into();
            }), // .map_err(|_| KernelError::ActixMailbox)
                // .from_err()
                // .and_then(move |file| {
                //     match file {
                //         Ok(file) => {
                //         let file = models::FileBody{
                //                 id: file.id,
                //                 created_at: file.created_at,
                //                 updated_at: file.updated_at,
                //                 name: file.name,
                //                 type_: file.type_,
                //                 size: file.size,
                //             };
                //             let res = api::Response::data(file);
                //             ok(HttpResponse::Created().json(&res))
                //         },
                //         Err(err) => {
                //             slog_error!(logger, "{}", err);
                //             ok(err.error_response())
                //         },
                //     }
                // })
    );
}

fn handle_upload(upload_path: &str, field: Field) -> Box<dyn Stream<Item = (), Error = Error>> {
    let file = match fs::File::create(upload_path) {
        Ok(file) => file,
        Err(e) => return Box::new(future::err(error::ErrorInternalServerError(e)).into_stream()),
    };

    return Box::new(
        Box::new(
            field
                .fold((file, 0u64), move |(mut file, mut acc), bytes| {
                    web::block(move || {
                        file.write_all(bytes.as_ref())
                            .map_err(|e| MultipartError::Payload(error::PayloadError::Io(e)))?;
                        acc += bytes.len() as u64;
                        if acc > crate::UPLOAD_MAX_SIZE {
                            return Err(MultipartError::Payload(error::PayloadError::Overflow));
                        }
                        Ok((file, acc))
                    })
                    .map_err(
                        |e: error::BlockingError<MultipartError>| match e {
                            error::BlockingError::Error(e) => e,
                            error::BlockingError::Canceled => MultipartError::Incomplete,
                        },
                    )
                })
                .map(|_| ())
                .map_err(error::ErrorInternalServerError),
        )
        .into_stream(),
    );
}

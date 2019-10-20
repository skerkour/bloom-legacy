use crate::controllers;
use actix_multipart::{Field, Multipart, MultipartError};
use actix_web::{error, web, Error, HttpRequest, HttpResponse, ResponseError};
use futures::{future, future::ok, future::Either, Future, IntoFuture, Stream};
use kernel::{
    api,
    api::middlewares::{GetRequestAuth, GetRequestId, GetRequestLogger},
    error::KernelError,
    log::macros::*,
};
use std::fs;
use std::io::Write;

pub fn post(
    multipart: Multipart,
    ids: web::Path<(uuid::Uuid, uuid::Uuid)>,
    state: web::Data<api::State>,
    req: HttpRequest,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;

    if auth.service.is_none() || auth.service.unwrap() != api::middlewares::Service::Phaser {
        return Either::A(ok(KernelError::Unauthorized(
            "Authentication required".to_string(),
        )
        .error_response()));
    }

    let report_dir = format!("tmp/phaser/reports/{}", ids.1); // ids.1 -> report_id
    match fs::create_dir_all(&report_dir) {
        Ok(_) => {}
        Err(err) => return Either::A(ok(KernelError::from(err).error_response())),
    }
    let report_file_path = format!("{}/report.zip", &report_dir);

    return Either::B(
        multipart
            .map_err(|err| KernelError::Internal(err.to_string()))
            .map(move |field| handle_upload(&report_file_path, field))
            .flatten()
            .collect()
            .into_future()
            .map_err(|_| KernelError::Validation("file too large".to_string()))
            .and_then(move |_| {
                state
                    .db
                    .send(controllers::CompleteReport {
                        report_dir,
                        scan_id: ids.0,
                        report_id: ids.1,
                        s3_bucket: state.config.s3.bucket.clone(),
                        s3_client: state.s3_client.clone(),
                        request_id,
                    })
                    .flatten()
            })
            .and_then(move |_| {
                let res = api::Response::data(api::NoData {});
                Ok(HttpResponse::Ok().json(&res))
            })
            .from_err()
            .map_err(move |err: KernelError| {
                slog_error!(logger, "{}", err);
                return err.into();
            }),
    );
}

fn handle_upload(report_path: &str, field: Field) -> Box<dyn Stream<Item = (), Error = Error>> {
    let file = match fs::File::create(report_path) {
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
                        if acc > crate::REPORT_MAX_SIZE {
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

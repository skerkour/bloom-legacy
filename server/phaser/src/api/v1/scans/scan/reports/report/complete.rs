use kernel::{
    api,
    log::macros::*,
    accounts,
    api::middlewares::{
        GetRequestLogger,
        GetRequestId,
        GetRequestAuth,
    },
    error::KernelError,
};
use futures::{
    Future,
    Stream,
    IntoFuture,
    future::ok,
    future,
    future::Either,
};
use actix_web::{
    web, Error, HttpRequest, HttpResponse, ResponseError, error,
};
use actix_multipart::{Field, Multipart, MultipartError};
use std::fs;
use std::io::Write;
use crate::{
    controllers,
    api::v1::models,
};


pub fn post(multipart: Multipart, state: web::Data<api::State>, req: HttpRequest)
-> impl Future<Item = HttpResponse, Error = Error> {
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;
    let report_file_path = "report.zip".to_string();
    let report_file_path2 = report_file_path.clone();

    if auth.service.is_none() || auth.service.unwrap() != api::middlewares::Service::Phaser {
        return Either::A(ok(KernelError::Unauthorized("Authentication required".to_string()).error_response()));
    }

    return Either::B(
        multipart
        .map_err(|err| KernelError::Internal(err.to_string()))
        .map(move |field| handle_upload(&report_file_path, field))
        .flatten()
        .collect()
        .into_future()
        .map_err(|_| KernelError::Validation("file too large".to_string()))
        .and_then(move |_| {
            state.db
            .send(controllers::CompleteReport{
                report_path: report_file_path2,
                request_id,
            }).flatten()
        })
        .and_then(move |_| {
            let res = api::Response::data(api::NoData{});
            Ok(HttpResponse::Ok().json(&res))
        })
        .from_err()
        .map_err(move |err: KernelError| {
            slog_error!(logger, "{}", err);
            return err.into();
        })
    );
}

fn handle_upload(report_path: &str, field: Field) -> Box<Stream<Item = (), Error = Error>> {
    let file = match fs::File::create(report_path) {
        Ok(file) => file,
        Err(e) => return Box::new(future::err(error::ErrorInternalServerError(e)).into_stream()),
    };

    return Box::new(
        Box::new(
            field.fold((file, 0u64), move |(mut file, mut acc), bytes| {
                web::block(move || {
                    file.write_all(bytes.as_ref()).map_err(|e| {
                        MultipartError::Payload(error::PayloadError::Io(e))
                    })?;
                    acc += bytes.len() as u64;
                    if acc > crate::REPORT_MAX_SIZE {
                        return Err(MultipartError::Payload(error::PayloadError::Overflow));
                    }
                    Ok((file, acc))
                })
                .map_err(|e: error::BlockingError<MultipartError>| {
                    match e {
                        error::BlockingError::Error(e) => e,
                        error::BlockingError::Canceled => MultipartError::Incomplete,
                    }
                })
            })
            .map(|(_)| ())
            .map_err(|e| error::ErrorInternalServerError(e)),
        ).into_stream()
    );
}

use crate::{
    api,
    log::macros::*,
    services::account::controllers,
    services::account::api::v1::models,
    api::middlewares::{
        GetRequestLogger,
        GetRequestId,
        GetRequestAuth,
    },
    error::KernelError,
};
use actix_web::{actix::System,server, App, AsyncResponder, Error, HttpMessage, FutureResponse,
HttpRequest, HttpResponse, Json, Result, middleware, State, dev, multipart, error, http::header::{ContentDisposition, HeaderMap}, Responder};
use serde::{Serialize, Deserialize};
use futures::{Future, Stream};
use futures::future;
use rand::Rng;
use std::fs;
use std::io::Write;

const MB2: usize = 2_097_152;

pub fn put(req: &HttpRequest<api::State>) -> FutureResponse<HttpResponse> {
    Box::new(
        req.multipart()
            .map_err(error::ErrorInternalServerError)
            .map(handle_multipart_item)
            .flatten()
            .collect()
            .map(|_| HttpResponse::Ok().json(models::NoData{}))
            .map_err(|e| {
                println!("failed: {}", e);
                e
            })
    )
}

fn handle_multipart_item(
    item: multipart::MultipartItem<dev::Payload>,
) -> Box<Stream<Item = i64, Error = Error>> {
    match item {
        multipart::MultipartItem::Field(field) => {
            println!("Field:");
            if let Some(cd) = field.content_disposition() {
                println!("FieldName: {:?}", cd.get_name());
                println!("FieldFileName: {:?}", cd.get_filename());
            }
            Box::new(save_file(field).into_stream())
        }
        multipart::MultipartItem::Nested(mp) => {
            println!("nested");
            Box::new(
            mp.map_err(error::ErrorInternalServerError)
                .map(handle_multipart_item)
                .flatten(),
            )
        },
    }
}

fn save_file(
    field: multipart::Field<dev::Payload>,
) -> Box<Future<Item = i64, Error = Error>> {
    let mut rng = rand::thread_rng();
    let x: u8 = rng.gen();
    let file_path_string = format!("avatar_{}.png", x);
    let mut file = match fs::File::create(file_path_string) {
        Ok(file) => file,
        Err(e) => return Box::new(future::err(error::ErrorInternalServerError(e))),
    };
    Box::new(
        field
        .fold(Vec::new(), |mut acc, bytes| -> future::FutureResult<_, error::MultipartError> {
            acc.extend_from_slice(&bytes);
            if acc.len() > MB2 {
                return future::err(error::MultipartError::Payload(error::PayloadError::Overflow))
            }
            future::ok(acc)
        })
        .and_then(move |buffer| {
            let len = file.write(&buffer)
                .map(|len| len as i64)
                .map_err(|e| {
                    println!("file.write failed: {:?}", e);
                    error::MultipartError::Payload(error::PayloadError::Io(e))
                });
            return future::result(len);
        })
        .map_err(|e| {
            println!("save_file failed, {:?}", e);
            error::ErrorInternalServerError(e)
        }),
    )
}

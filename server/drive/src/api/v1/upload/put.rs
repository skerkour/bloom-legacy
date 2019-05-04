use kernel::{
    api,
    log::macros::*,
    api::middlewares::{
        GetRequestLogger,
        GetRequestId,
        GetRequestAuth,
    },
    KernelError,
};
use futures::{
    future::Future,
    future::ok,
    future::Either,
};
use actix_web::{
    web, Error, HttpRequest, HttpResponse, ResponseError,
};
use crate::{
    api::v1::models,
    controllers,
};


pub fn put(upload_data: web::Json<models::CompleteUploadSessionBody>, state: web::Data<api::State>, req: HttpRequest)
-> impl Future<Item = HttpResponse, Error = Error> {
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;

    if auth.session.is_none() || auth.account.is_none() {
        return Either::A(ok(KernelError::Unauthorized("Authentication required".to_string()).error_response()));
    }

    return Either::B(
        state.db
        .send(controllers::CompleteUploadSession{
            upload_session_id: upload_data.upload_session_id.clone(),
            s3_bucket: state.config.s3_bucket(),
            s3_client: state.s3_client.clone(),
            account_id: auth.account.expect("error unwraping non none account").id,
            session_id: auth.session.expect("error unwraping non none session").id,
            request_id,
        })
        .from_err()
        .and_then(move |file| {
            match file {
                Ok(file) => {
                let file = models::FileBody{
                        id: file.id,
                        created_at: file.created_at,
                        updated_at: file.updated_at,
                        name: file.name,
                        type_: file.type_,
                        size: file.size,
                    };
                    let res = api::Response::data(file);
                    ok(HttpResponse::Created().json(&res))
                },
                Err(err) => {
                    slog_error!(logger, "{}", err);
                    ok(err.error_response())
                },
            }
        })
    );
}

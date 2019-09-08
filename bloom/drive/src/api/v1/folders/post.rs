use crate::{api::v1::models, controllers};
use actix_web::{web, Error, HttpRequest, HttpResponse, ResponseError};
use futures::{future::ok, future::Either, future::Future};
use kernel::{
    api,
    api::middlewares::{GetRequestAuth, GetRequestId, GetRequestLogger},
    log::macros::*,
    KernelError,
};

pub fn post(
    create_data: web::Json<models::CreateFolderBody>,
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
        state
            .db
            .send(controllers::CreateFolder {
                parent_id: create_data.parent_id,
                name: create_data.name.clone(),
                owner_id: auth.account.expect("error unwraping non none account").id,
                session_id: auth.session.expect("error unwraping non none session").id,
                request_id,
            })
            .map_err(|_| KernelError::ActixMailbox)
            .from_err()
            .and_then(move |new_folder| match new_folder {
                Ok(new_folder) => {
                    let res = models::FileBody {
                        id: new_folder.id,
                        created_at: new_folder.created_at,
                        updated_at: new_folder.updated_at,
                        name: new_folder.name,
                        type_: new_folder.type_,
                        size: new_folder.size,
                    };
                    let res = api::Response::data(res);
                    ok(HttpResponse::Created().json(&res))
                }
                Err(err) => {
                    slog_error!(logger, "{}", err);
                    ok(err.error_response())
                }
            }),
    );
}

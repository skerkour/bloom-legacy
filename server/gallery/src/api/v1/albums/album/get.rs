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
    album_id: web::Path<(uuid::Uuid)>,
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
            .send(controllers::FindAlbum {
                album_id: album_id.into_inner(),
                s3_bucket: state.config.s3.bucket.clone(),
                s3_region: state.config.aws.region.clone(),
                account_id: auth.account.expect("unwrapping non none account").id,
            })
            .map_err(|_| KernelError::ActixMailbox)
            .from_err()
            .and_then(move |res| match res {
                Ok((album, media)) => {
                    let res = models::AlbumWithMediaResponse {
                        album: From::from(album),
                        media,
                    };
                    let res = api::Response::data(res);
                    ok(HttpResponse::Ok().json(&res))
                }
                Err(err) => {
                    slog_error!(logger, "{}", err);
                    ok(err.error_response())
                }
            }),
    );
}

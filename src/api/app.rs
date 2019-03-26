use crate::{
    db::DbActor,
    api,
    api::middlewares,
    config,
    services::account::api::v1 as accountv1,
};
use actix_web::{
    App,
    middleware::cors::Cors,
    http::{header},
};
use rusoto_core::Region;
use rusoto_s3::S3Client;
use std::str::FromStr;

pub fn init(db: actix::Addr<DbActor>, cfg: config::Config) -> App<api::State> {

    let region = Region::from_str(&cfg.aws_region()).expect("AWS region not valid");
    let api_state = api::State{
        db,
        config: cfg,
        s3_client: S3Client::new(region),
    };

    App::with_state(api_state.clone())
    .middleware(middlewares::RequestIdMiddleware)
    .middleware(middlewares::LoggerMiddleware)
    .middleware(middlewares::DefaultHeaders)
    .middleware(
        // cors 2 times because otherwise authmiddleware doesn't works...
        Cors::build()
        .send_wildcard()
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![header::ORIGIN, header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
        .max_age(3600)
        .finish()
    )
    .middleware(middlewares::AuthMiddleware)
    .default_resource(|r| r.f(api::route_404))
    .configure(|app| {
        Cors::for_app(app)
            // .allowed_origin("*")
            .send_wildcard()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::ORIGIN, header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
            .max_age(3600)
            .resource("/", |r| r.method(http::Method::GET).f(api::index))
            .resource("/account/v1/welcome/register", |r| r.method(http::Method::POST).with_config(accountv1::register::post, api::json_default_config))
            .resource("/account/v1/welcome/verify", |r| r.method(http::Method::POST).with_config(accountv1::verify::post, api::json_default_config))
            .resource("/account/v1/welcome/complete", |r| r.method(http::Method::POST).with_config(accountv1::complete_registration::post, api::json_default_config))
            .resource("/account/v1/sign-in", |r| r.method(http::Method::POST).with_config(accountv1::sign_in::post, api::json_default_config))
            .resource("/account/v1/sign-out", |r| r.method(http::Method::POST).f(accountv1::sign_out::post))
            .resource("/account/v1/recover", |r| r.method(http::Method::POST).with_config(accountv1::recover::post, api::json_default_config))
            .resource("/account/v1/recover", |r| r.method(http::Method::PUT).with_config(accountv1::recover::put, api::json_default_config))
            .resource("/account/v1/me", |r| {
                r.method(http::Method::GET).f(accountv1::me::get);
                r.method(http::Method::PUT).with_config(accountv1::me::put, api::json_default_config);
            })
            .resource("/account/v1/me/password", |r| r.method(http::Method::PUT).with_config(accountv1::me::password::put, api::json_default_config))
            .resource("/account/v1/me/avatar", |r| r.method(http::Method::PUT).f(accountv1::me::avatar::put))
            .resource("/account/v1/me/email", |r| r.method(http::Method::PUT).with_config(accountv1::me::email::put, api::json_default_config))
            .resource("/account/v1/me/email/verify", |r| r.method(http::Method::POST).with_config(accountv1::me::email::verify::post, api::json_default_config))
            .resource("/account/v1/me/sessions", |r| r.method(http::Method::GET).f(accountv1::me::sessions::get))
            .resource("/account/v1/me/sessions/{session_id}/revoke", |r| r.method(http::Method::POST).with(accountv1::me::sessions::revoke::post))
            .register()
    })
}

use crate::{
    db::DbActor,
    api,
    api::middlewares,
    services,
    config,
};
use actix_web::{
    App,
    middleware::cors::Cors,
    http::{header},
};


pub fn init(db: actix::Addr<DbActor>, cfg: config::Config) -> App<api::State> {
    let api_state = api::State{
        db,
        config: cfg,
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
            .resource("/account/v1/welcome/register", |r| r.method(http::Method::POST).with_config(services::account::api::v1::register_post, api::json_default_config))
            .resource("/account/v1/welcome/verify", |r| r.method(http::Method::POST).with_config(services::account::api::v1::verify_post, api::json_default_config))
            .resource("/account/v1/welcome/complete", |r| r.method(http::Method::POST).with_config(services::account::api::v1::complete_registration_post, api::json_default_config))
            .resource("/account/v1/welcome/complete", |r| r.method(http::Method::POST).with_config(services::account::api::v1::complete_registration_post, api::json_default_config))
            .resource("/account/v1/sign-in", |r| r.method(http::Method::POST).with_config(services::account::api::v1::sign_in_post, api::json_default_config))
            .resource("/account/v1/sign-out", |r| r.method(http::Method::POST).f(services::account::api::v1::sign_out_post))
            .register()
    })
}

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
    .middleware(middlewares::request_id::RequestIDHeader)
    .middleware(middlewares::logger::Logger)
    .middleware(middlewares::DefaultHeaders)
    .default_resource(|r| r.f(api::route_404))
    .configure(|app| {
        Cors::for_app(app)
            // .allowed_origin("*")
            .send_wildcard()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::ORIGIN, header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
            .max_age(3600)
            .resource("/", |r| r.method(http::Method::GET).f(api::index))
            .resource("/account/v1/register", |r| r.method(http::Method::POST).with_config(services::account::api::v1::post_register, api::json_default_config))
            .register()
    })
}

use crate::{
    db::DbActor,
    api,
    api::middlewares,
    config,
    services::account::api::v1 as accountv1,
    services::notes::api::v1 as notesv1,
    services::contacts::api::v1 as contactsv1,
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
            // account
            .resource("/account/v1/registration/start", |r| r.method(http::Method::POST).with_config(accountv1::registration::start::post, api::json_default_config))
            .resource("/account/v1/registration/verify", |r| r.method(http::Method::POST).with_config(accountv1::registration::verify::post, api::json_default_config))
            .resource("/account/v1/registration/complete", |r| r.method(http::Method::POST).with_config(accountv1::registration::complete::post, api::json_default_config))
            .resource("/account/v1/sign-in", |r| r.method(http::Method::POST).with_config(accountv1::sign_in::post, api::json_default_config))
            .resource("/account/v1/sign-out", |r| r.method(http::Method::POST).f(accountv1::sign_out::post))
            .resource("/account/v1/recover", |r| {
                r.method(http::Method::POST).with_config(accountv1::recover::post, api::json_default_config);
                r.method(http::Method::PUT).with_config(accountv1::recover::put, api::json_default_config);
            })
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

            // notes
            .resource("/notes/v1/notes", |r| {
                r.method(http::Method::GET).f(notesv1::notes::get);
                r.method(http::Method::POST).with_config(notesv1::notes::post, api::json_default_config);
            })
            .resource("/notes/v1/notes/{note_id}", |r| {
                r.method(http::Method::DELETE).with(notesv1::notes::delete);
                r.method(http::Method::PUT).with_config(notesv1::notes::put, api::json_default_config_path);
            })
            .resource("/notes/v1/notes/{note_id}/archive", |r| r.method(http::Method::POST).with(notesv1::notes::archive::post))
            .resource("/notes/v1/notes/{note_id}/unarchive", |r| r.method(http::Method::POST).with(notesv1::notes::unarchive::post))
            .resource("/notes/v1/notes/{note_id}/remove", |r| r.method(http::Method::POST).with(notesv1::notes::remove::post))
            .resource("/notes/v1/notes/{note_id}/restore", |r| r.method(http::Method::POST).with(notesv1::notes::restore::post))
            .resource("/notes/v1/archive", |r| r.method(http::Method::GET).f(notesv1::archive::get))
            .resource("/notes/v1/trash", |r| r.method(http::Method::GET).f(notesv1::trash::get))

            // contacts
            .resource("/contacts/v1/contacts", |r| {
                r.method(http::Method::GET).f(contactsv1::contacts::get);
                r.method(http::Method::POST).with_config(contactsv1::contacts::post, api::json_default_config);
            })
            .resource("/contacts/v1/contacts/{contact_id}", |r| {
                r.method(http::Method::GET).with(contactsv1::contacts::id::get);
                r.method(http::Method::PUT).with(contactsv1::contacts::put);
                r.method(http::Method::DELETE).with(contactsv1::contacts::delete);
            })


            .register()
    })
}

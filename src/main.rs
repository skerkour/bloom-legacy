mod app;

use actix::System;
use actix_cors::Cors;
use actix_web::{
    http::header, middleware, middleware::Logger, middleware::NormalizePath, web, App, HttpServer,
};
use bloom_kernel::{
    api,
    config,
    db,
    log,
    log::macros::{slog_info, slog_o},
    // myaccount::domain::account,
};
use rusoto_core::Region;
use rusoto_s3::S3Client;
use sentry::integrations::panic::register_panic_handler;
use std::env;
use std::str::FromStr;

fn register_reactors() {
    // eventsourcing::subscribe::<_, account::Created, _>(Box::new(
    //     drive::reactors::AccountCreated {},
    // ));
    // eventsourcing::subscribe::<_, account::Created, _>(Box::new(
    //     bitflow::reactors::AccountCreated {},
    // ));
    // eventsourcing::subscribe::<_, account::Event, _>(Box::new(
    //     billing::reactors::AccountCreated {},
    // ));
}

fn main() {
    let cfg = config::init();
    let _sentry_guard = sentry::init(cfg.sentry.server_url.clone());
    env::set_var("RUST_BACKTRACE", "1");
    register_panic_handler();

    let (_log_guard, logger) = log::setup_slog();

    crypto42::init().expect("error initializing crypto42");

    let sys = System::new("bloom_server");
    let db_actor_addr = db::init(&cfg);
    let binding_addr = format!("0.0.0.0:{}", &cfg.port);

    register_reactors();

    let region = Region::from_str(&cfg.aws.region).expect("AWS region not valid");
    let api_state = api::State {
        db: db_actor_addr,
        config: cfg.clone(),
        s3_client: S3Client::new(region),
    };

    // TODO: logger middleware
    // TODO: sentry middleware
    HttpServer::new(move || {
        App::new()
        .data(api_state.clone())
        .wrap(api::middlewares::AuthMiddleware)
        .wrap(Cors::new()
            .allowed_origin(&cfg.host)
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::ORIGIN, header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
            .max_age(3600)
        )
        .wrap(NormalizePath)
        .wrap(Logger::new(r#""%r" %s %b %D"#))
        .wrap(api::middlewares::RequestIdMiddleware)
        .wrap(middleware::DefaultHeaders::new()
            .header("X-Content-Type-Options", "nosniff")
            .header("X-Frame-Options", "Deny")
            .header("Strict-Transport-Security", "max-age=63072000; includeSubDomains; preload")
            .header("X-XSS-Protection", "1; mode=block")
            .header("Expect-CT", "max-age=86400; enforce")
            .header("Content-Security-Policy", "default-src 'self' https://s3.eu-west-3.amazonaws.com https://s3-eu-west-3.amazonaws.com; script-src 'self' 'unsafe-inline' 'unsafe-eval'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https://s3.eu-west-3.amazonaws.com https://s3-eu-west-3.amazonaws.com; connect-src 'self';")
        )

        .configure(app::config(cfg.clone()))
        .default_service(web::route().to(api::route_404))
    })
    .backlog(8192)
    .keep_alive(60)
    .shutdown_timeout(2)
    .workers(num_cpus::get() * 2)
    .bind(&binding_addr)
    .expect("error binding server")
    .start();

    slog_info!(logger, "server started"; slog_o!("address" => binding_addr));
    let _ = sys.run();
}

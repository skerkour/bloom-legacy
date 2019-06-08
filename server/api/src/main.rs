mod app;


use std::env;
use dotenv;
use sentry::integrations::panic::register_panic_handler;
use actix_web::{
    http::header, middleware::cors::Cors, middleware::Logger, middleware::NormalizePath, web, App, HttpServer, middleware,
};
use rusoto_core::Region;
use rusoto_s3::S3Client;
use actix::System;
use kernel::{
    log,
    log::macros::{slog_info, slog_o},
    config,
    db,
    api,
    myaccount::domain::account,
};
use std::str::FromStr;


fn register_reactors() {
    eventsourcing::subscribe::<_, account::Event, _>(Box::new(drive::reactors::AccountCreated{}));
    eventsourcing::subscribe::<_, account::Event, _>(Box::new(bitflow::reactors::AccountCreated{}));
    eventsourcing::subscribe::<_, account::Event, _>(Box::new(billing::reactors::AccountCreated{}));
}

fn main() {
    dotenv::dotenv().ok();
    let _sentry_guard = sentry::init(env::var("SENTRY_URL").unwrap());
    env::set_var("RUST_BACKTRACE", "1");
    register_panic_handler();

    let (_log_guard, logger) = log::setup_slog();

    let sys = System::new("kernel");
    let cfg = config::init();
    let db_actor_addr = db::init(&cfg);
    let binding_addr = format!("0.0.0.0:{}", cfg.port());

    register_reactors();

    let region = Region::from_str(&cfg.aws_region()).expect("AWS region not valid");
    let api_state = api::State{
        db: db_actor_addr,
        config: cfg,
        s3_client: S3Client::new(region),
    };

    // TODO: logger middleware
    // TODO: sentry middleware
    HttpServer::new(move || {
        App::new()
        .data(api_state.clone())
        .wrap(api::middlewares::AuthMiddleware)
        .wrap(
            Cors::new()
                .allowed_origin("https://bloom.sh")
                .allowed_origin("https://www.bloom.sh")
                .allowed_origin("http://localhost:8080")
                .allowed_origin("http://127.0.0.1:8080")
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![header::ORIGIN, header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
                .max_age(3600)
        )
        .wrap(NormalizePath)
        .wrap(Logger::default())
        .wrap(api::middlewares::RequestIdMiddleware)
        .wrap(middleware::DefaultHeaders::new()
            .header("X-Content-Type-Options", "nosniff")
            .header("X-Frame-Options", "Deny")
            .header("Strict-Transport-Security", "max-age=63072000; includeSubDomains; preload")
            .header("X-XSS-Protection", "1; mode=block")
            .header("Expect-CT", "max-age=86400; enforce")
        )

        .configure(app::config)
        .default_service(web::route().to(app::p404))
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

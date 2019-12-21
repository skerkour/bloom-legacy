use actix::System;
use actix_cors::Cors;
use actix_web::{
    http::header, middleware, middleware::Logger, middleware::NormalizePath, web, App, HttpServer,
};
use bloom_kernel::{
    api,
    config,
    db,
    log::macros::{slog_info, slog_o},
    // myaccount::domain::account,
};
use rusoto_core::Region;
use rusoto_s3::S3Client;
use std::error::Error;
use std::str::FromStr;

pub fn run(cfg: config::Config, logger: slog::Logger) -> Result<(), Box<dyn Error>> {
    let sys = System::new("bloom_server");
    let db_actor_addr = db::init(&cfg);
    let binding_addr = format!("0.0.0.0:{}", cfg.port);

    let region = Region::from_str(&cfg.aws.region)?;
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

        .configure(crate::app::config(cfg.clone()))
        .default_service(web::route().to(api::route_404))
    })
    .backlog(8192)
    .keep_alive(60)
    .shutdown_timeout(2)
    .workers(num_cpus::get() * 2)
    .bind(&binding_addr)?
    .start();

    slog_info!(logger, "server started"; slog_o!("address" => binding_addr, "version" => crate::info::VERSION));
    let _ = sys.run();

    return Ok(());
}

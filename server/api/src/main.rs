mod app;
use std::env;
use dotenv;

use sentry::integrations::panic::register_panic_handler;


use actix_web::{
    server as actix_server,
};
use actix::System;
use kernel::{
    log,
    log::macros::{slog_info, slog_o},
    config,
    db,
    accounts::domain::account,
};


fn register_reactors() {
    eventsourcing::subscribe::<_, account::Event, _>(Box::new(drive::reactors::AccountCreated{}));
    eventsourcing::subscribe::<_, account::Event, _>(Box::new(bitflow::reactors::AccountCreated{}));
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

    actix_server::new(move || app::init(db_actor_addr.clone(), cfg.clone()))
        .backlog(8192)
        .bind(&binding_addr)
        .expect("error binding server")
        .keep_alive(actix_server::KeepAlive::Timeout(60))
        .shutdown_timeout(2)
        .workers(num_cpus::get())
        .start();

    slog_info!(logger, "server started"; slog_o!("address" => binding_addr));
    let _ = sys.run();
}

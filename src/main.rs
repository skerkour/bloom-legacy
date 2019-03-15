#[macro_use]
extern crate failure;
#[macro_use]
extern crate diesel;


mod db;
mod log;
mod api;
mod services;
mod error;
mod config;


use actix_web::{
    server as actix_server,
};
use actix::System;
use serde::{Serialize, Deserialize};
use crate::{
    log::macros::{slog_info, slog_o}
};



#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewUserRequest {
    pub name: String,
}



fn main() {
    let (_guard, logger) = log::setup_slog();
    let sys = System::new("kernel");
    let cfg = config::init();
    let db_actor_addr = db::init(&cfg);
    let binding_addr = format!("0.0.0.0:{}", cfg.port());

    actix_server::new(move || api::init(db_actor_addr.clone(), cfg.clone()))
    .backlog(8192)
    .bind(&binding_addr)
    .unwrap()
    .keep_alive(actix_server::KeepAlive::Timeout(60))
    .shutdown_timeout(2)
    .workers(num_cpus::get())
    .start();

    slog_info!(logger, "server started"; slog_o!("address" => binding_addr));
    let _ = sys.run();
}

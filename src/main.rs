mod app;
mod info;
mod server;

use bloom_kernel::{
    config,
    log,
    // myaccount::domain::account,
};
use clap::{App as ClapApp, SubCommand};
use sentry::integrations::panic::register_panic_handler;
use std::env;
use std::error::Error;

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

fn main() -> Result<(), Box<dyn Error>> {
    // init config
    let cfg = config::init();

    // init sentry (error tracking)
    let _sentry_guard = sentry::init(cfg.sentry.server_url.clone());
    env::set_var("RUST_BACKTRACE", "1");
    register_panic_handler();

    // init other parts
    let (_log_guard, logger) = log::setup_slog();
    crypto42::init()?;
    register_reactors();

    let matches = ClapApp::new(info::NAME)
        .version(info::VERSION)
        .author(info::AUTHOR)
        .about(info::ABOUT)
        .subcommand(SubCommand::with_name("server").about("run the server"))
        .get_matches();

    if matches.subcommand_matches("server").is_some() {
        return server::run(cfg, logger);
    }

    return Ok(());
}

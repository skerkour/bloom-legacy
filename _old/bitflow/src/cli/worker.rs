use crate::worker::{config::Config, Worker};
use clap::ArgMatches;
use sentry;
use std::env;
use std::sync::mpsc;
use std::{thread, time};

// we start sentry here, because tracking stop when guard go out of scope, so it can't be in Worker::new
pub fn run(_: &ArgMatches) -> Result<(), String> {
    env::set_var("RUST_BACKTRACE", "1");
    let config = Config::new();
    let _sentry = sentry::init(config.sentry_url.clone());
    sentry::integrations::panic::register_panic_handler();

    let (tx, rx) = mpsc::channel();
    for i in 0..config.concurrency {
        let tx1 = mpsc::Sender::clone(&tx);
        let cfg = config.clone();
        thread::spawn(move || {
            Worker::new(cfg, u64::from(i)).run();
            tx1.send(i).unwrap();
        });
        thread::sleep(time::Duration::from_secs(1))
    }

    for received in rx {
        println!("worker: {} terminated", received);
    }
    Ok(())
}

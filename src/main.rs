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

    use lettre::{EmailTransport, SmtpTransport};
    use lettre::smtp::authentication::{Credentials, Mechanism};
    use lettre_email::EmailBuilder;

    let email = EmailBuilder::new()
        // Addresses can be specified by the tuple (email, alias)
        .to(("sylvain.kerkour@gmail.com", "Sylvain Kerkour"))
        // ... or by an address only
        .from("hello@bloom.sh")
        .subject("Your bloom activation code: 123-123")
        .text("voici ton code pour activer tom compte: 123-3ptits-chats")
        .build()
        .unwrap();

    let mut mailer = SmtpTransport::simple_builder(std::env::var("SMTP_HOST").unwrap().as_str()).unwrap()
    // .hello_name(ClientId::Domain("my.hostname.tld".to_string()))
        // Add credentials for authentication
        .credentials(Credentials::new(std::env::var("SMTP_USERNAME").unwrap(), std::env::var("SMTP_PASSWORD").unwrap()))
        // Enable SMTPUTF8 if the server supports it
        .smtp_utf8(true)
    .build();
    // Send the email
    let result = mailer.send(&email);

    if result.is_ok() {
        println!("Email sent");
    } else {
        println!("Could not send email: {:?}", result);
    }

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

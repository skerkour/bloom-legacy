mod default_template;

use crate::config::Config;
use lettre::{
    SmtpClient,
    Transport,
    smtp::authentication::Credentials,
};
use lettre_email::EmailBuilder;


pub use default_template::DEFAULT_TEMPLATE;


pub fn send_email(config: &Config, from: (&str, &str), to: (&str, &str), subject: &str, content: &str) {

    let email = EmailBuilder::new()
        // Addresses can be specified by the tuple (email, alias)
        .to(to)
        // ... or by an address only
        .from(from)
        .subject(subject)
        .html(content)
        .build()
        .expect("error building email");

    let mut mailer = SmtpClient::new_simple(config.smtp.host.as_str()).expect("error building email transport")
        // Add credentials for authentication
        .credentials(Credentials::new(config.smtp.username.clone(), config.smtp.password.clone()))
        // Enable SMTPUTF8 if the server supports it
        .smtp_utf8(true)
        .transport();

    // Send the email
    mailer.send(email.into()).expect("error sending email");
}

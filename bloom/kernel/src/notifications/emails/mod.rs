mod default_template;

use crate::config::Config;
use lettre::{EmailTransport, SmtpTransport};
use lettre::smtp::authentication::{Credentials};
use lettre_email::EmailBuilder;

pub use default_template::DEFAULT_TEMPLATE;
pub static HELLO_ADDRESS: &str = "hello@bloom.sh";
pub static WELCOME_ADDRESS: &str = "welcome@bloom.sh";
pub static NOTIFY_ADDRESS: &str = "notify@bloom.sh";
pub static SECURITY_ADDRESS: &str = "security@bloom.sh";


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

    let mut mailer = SmtpTransport::simple_builder(config.smtp_host().as_str()).expect("error building emai ltransport")
        // Add credentials for authentication
        .credentials(Credentials::new(config.smtp_username(), config.smtp_password()))
        // Enable SMTPUTF8 if the server supports it
        .smtp_utf8(true)
        .build();

    // Send the email
    mailer.send(&email).expect("error sending email");
}

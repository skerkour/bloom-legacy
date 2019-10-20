mod default_template;

use crate::config::Config;
use bloom_error::BloomError;
use lettre::{smtp::authentication::Credentials, SmtpClient, Transport};
use lettre_email::EmailBuilder;

pub use default_template::DEFAULT_TEMPLATE;

pub fn send_email(
    config: &Config,
    from: (&str, &str),
    to: (&str, &str),
    subject: &str,
    content: &str,
) -> Result<(), BloomError> {
    // Useful in development mode when you haven't a smtp configured
    if config.smtp.host == "" {
        println!("=============\n{}\n=============", content);
        return Ok(());
    }
    let email = EmailBuilder::new()
        // Addresses can be specified by the tuple (email, alias)
        .to(to)
        // ... or by an address only
        .from(from)
        .subject(subject)
        .html(content)
        .build()?;

    let mut mailer = SmtpClient::new_simple(config.smtp.host.as_str())?
        // Add credentials for authentication
        .credentials(Credentials::new(
            config.smtp.username.clone(),
            config.smtp.password.clone(),
        ))
        // Enable SMTPUTF8 if the server supports it
        .smtp_utf8(true)
        .transport();

    // Send the email
    mailer.send(email.into())?;
    return Ok(());
}

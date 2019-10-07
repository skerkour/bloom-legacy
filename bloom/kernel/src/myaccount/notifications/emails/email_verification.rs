use crate::{config::Config, notifications::emails};
use bloom_error::BloomError;
use handlebars::Handlebars;
use std::collections::BTreeMap;

static TEMPLATE: &str = r#"
Your confirmation code to confirm the ownership of <b>{{new_email}}</b> is: <br/>
<h2>{{code}}</h2> <br/>
This code will only be valid for 30 minutes. <br/>
<hr/>
If you did not ask for a code, please ignore this email.
"#;

pub fn send_email_verification_code(
    config: &Config,
    email: &str,
    recipient_name: &str,
    new_email: &str,
    code: &str,
) -> Result<(), BloomError> {
    let mut formatted_code = code.to_string();
    formatted_code.insert(4, '-');
    let handlebars = Handlebars::new();

    let subject = format!("Confirmation code: {}", formatted_code);

    let mut data = BTreeMap::new();
    data.insert("code".to_string(), formatted_code);
    data.insert("new_email".to_string(), new_email.to_string());

    emails::send_email(
        config,
        (bloom_const::ADDRESS_NOTIFY, "Bloom"),
        (email, recipient_name),
        &subject,
        handlebars
            .render_template(TEMPLATE, &data)
            .expect("error rendering template")
            .as_str(),
    )?;

    return Ok(());
}

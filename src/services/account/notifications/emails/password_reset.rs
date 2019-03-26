use crate::{
    config::Config,
    error::KernelError,
    services::common::notifications::emails,
};
use std::collections::BTreeMap;
use handlebars::Handlebars;

static TEMPLATE: &str = r#"
<a href="{{url}}">Click here to reset your password</a>.<br/>
This link will expire in 30 minutes.
"#;


pub fn send_password_reset(config: &Config, email: &str, recipient_name: &str,
password_reset_id: &str, token: &str) -> Result<(), KernelError> {

    let password_reset_url = format!("{}/recovery?id={}&token={}", config.www_host(), password_reset_id, token);
    let handlebars = Handlebars::new();

    let subject = "Your password reset link";

    let mut data = BTreeMap::new();
    data.insert("url".to_string(), password_reset_url);

    emails::send_email(
        config,
        (emails::NOTIFY_ADDRESS, "Bloom"),
        (email, recipient_name),
        subject,
        handlebars.render_template(TEMPLATE, &data).expect("error rendering template").as_str(),
    );

    return Ok(());
}


// func SendAccountRecovery(email, firstName, lastName, id, token string) error {
// 	escapedID := url.QueryEscape(id)
// 	escapedToken := url.QueryEscape(token)
// 	passwordResetURL := config.WWWHost + "/recovery?id=" + escapedID + "&token=" + escapedToken

// 	from := mail.NewEmail("Bloom", NotifySenderAddress)
// 	subject := "Your password reset link"
// 	to := mail.NewEmail(firstName+" "+lastName, email)
// 	content := mail.NewContent("text/html", fmt.Sprintf(`
// 		<a href="%s">Click here to reset your password</a>, this link will expire in 30 minutes.
// 	`, passwordResetURL))
// 	m := mail.NewV3MailInit(from, subject, to, content)
// 	m.SetTemplateID(DefaultTemplateID)
// 	m.Personalizations[0].SetSubstitution("{{subject}}", subject)

// 	client := sendgrid.NewSendClient(config.SendgridAPIKey)
// 	_, err := client.Send(m)

// 	return err
// }

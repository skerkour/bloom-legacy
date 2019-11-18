use crate::{config::Config, notifications::emails};
use bloom_error::BloomError;
use handlebars::Handlebars;
use std::collections::BTreeMap;

static TEMPLATE: &str = r#"
Welcome aboard! <br/>
Your confirmation code is: <br/>
<h2>{{formatted_code}}</h2> <br/>
This code will only be valid for 30 minutes. <br/>
If you did not ask for a code, please ignore this email.
"#;

pub fn send_account_verification_code(
    config: &Config,
    email: &str,
    recipient_name: &str,
    code: &str,
) -> Result<(), BloomError> {
    let mut formatted_code = code.to_string();
    formatted_code.insert(4, '-');
    let handlebars = Handlebars::new();

    let subject = format!("Confirmation code: {}", formatted_code);

    let mut data = BTreeMap::new();
    data.insert("formatted_code".to_string(), formatted_code);

    emails::send_email(
        config,
        (bloom_const::ADDRESS_HELLO, "Bloom"),
        (email, recipient_name),
        &subject,
        handlebars
            .render_template(TEMPLATE, &data)
            .expect("error rendering template")
            .as_str(),
    )?;

    // escapedID := url.QueryEscape(id)
    // formattedToken := token[:4] + "-" + token[4:]
    // verifyURL := config.WWWHost + "/welcome/verify?id=" + escapedID + "&token=" + token

    // from := mail.NewEmail("Bloom", HelloSenderAddress)
    // subject := fmt.Sprintf("Confirmation code: %s", formattedToken)
    // to := mail.NewEmail(firstName+" "+lastName, email)
    // content := mail.NewContent(
    // 	"text/html",
    // 	fmt.Sprintf(`Welcome aboard! <br/>
    // 		Your confirmation code is: <br/>
    // 		<h2>%s</h2> <br/>
    // 		This code will only be valid for 30 minutes. <br/>
    // 		If you did not ask for a code, please ignore this email.
    // 		<hr>
    // 		You can also use the following link to verify your account: <br/>
    // 		<a href="%s">%s</a>
    // 	`, formattedToken, verifyURL, verifyURL),
    // )
    // m := mail.NewV3MailInit(from, subject, to, content)
    // m.Personalizations[0].SetSubstitution("{{subject}}", subject)
    // m.SetTemplateID(DefaultTemplateID)

    // client := sendgrid.NewSendClient(config.SendgridAPIKey)
    // _, err := client.Send(m)

    return Ok(());
}

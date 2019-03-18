use crate::{
    config::Config,
    error::KernelError,
    services::common::notifications::emails,
};
use std::collections::BTreeMap;
use handlebars::Handlebars;

static TEMPLATE: &str = r#"
Welcome aboard! <br/>
Your confirmation code is: <br/>
<h2>{{formatted_code}}</h2> <br/>
This code will only be valid for 30 minutes. <br/>
If you did not ask for a code, please ignore this email.
<hr>
You can also use the following link to verify your account: <br/>
<a href="{{url}}">{{url}}</a>
"#;


pub fn send_account_verification_code(config: &Config, email: &str, recipient_name: &str,
pending_account_id: &str, code: &str) -> Result<(), KernelError> {

    let mut formatted_code = code.to_string();
    formatted_code.insert(4, '-');
    let handlebars = Handlebars::new();
    let url = format!("{}/welcome/verify?id={}&code={}", config.www_host(), pending_account_id, code);

    let subject = format!("Confirmation code: {}", formatted_code);

    let mut data = BTreeMap::new();
    data.insert("formatted_code".to_string(), formatted_code.clone());
    data.insert("url".to_string(), url);

    emails::send_email(
        config,
        (emails::HELLO_ADDRESS, "Bloom"),
        (email, recipient_name),
        &subject,
        handlebars.render_template(TEMPLATE, &data).expect("error rendering template").as_str(),
    );


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

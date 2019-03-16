use crate::{
    config::Config,
    error::KernelError,
};
use lettre::{EmailTransport, SmtpTransport};
use lettre::smtp::authentication::{Credentials};
use lettre_email::EmailBuilder;
use std::collections::BTreeMap;
use handlebars::Handlebars;

static template: &str = r#"
Welcome aboard! <br/>
Your confirmation code is: <br/>
<h2>{{formatted_code}}</h2> <br/>
This code will only be valid for 30 minutes. <br/>
If you did not ask for a code, please ignore this email.
<hr>
You can also use the following link to verify your account: <br/>
<a href="{{url}}">{{url}}</a>
"#;


pub fn send_account_verification_code(config: &Config, email: &str, first_name: &str, last_name: &str,
pending_account_id: &str, code: &str) -> Result<(), KernelError> {

    let mut formatted_code = code.to_string();
    formatted_code.insert(3, '-');
    let mut handlebars = Handlebars::new();
    let url = format!("{}/welcome/verify?id={}&code={}", config.www_host(), pending_account_id, code);

    handlebars.register_template_string("t1", template).expect("error registrating template");

    let mut data = BTreeMap::new();
    data.insert("formatted_code".to_string(), formatted_code);
    data.insert("url".to_string(), url);

    let email = EmailBuilder::new()
        // Addresses can be specified by the tuple (email, alias)
        .to(("sylvain.kerkour@gmail.com", "Sylvain Kerkour"))
        // ... or by an address only
        .from("hello@bloom.sh")
        .subject("Your bloom activation code: 123-123")
        .html(handlebars.render("t1", &data).expect("error rendering template"))
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

use crate::error::KernelError;

pub fn send_account_verification_code(email: &str, first_name: &str, last_name: &str,
pending_account_id: &str, code: &str) -> Result<(), KernelError> {

    let formatted_code = code.to_string().insert(3, '-');
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

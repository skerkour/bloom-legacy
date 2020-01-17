package notification

import (
	"fmt"
	"net/smtp"
)

var emailer mailer

type mailer struct {
	address string
	auth    smtp.Auth
}

func SendHTMLEmail(fromAddr, fromName, toAddr, toName, subject, content string) error {
	toList := []string{toAddr}

	data := fmt.Sprintf(`From: %s <%s>
To: %s <%s>
Subject: %s
MIME-version: 1.0;
Content-Type: text/html; charset="utf-8";

%s
`, fromName, fromAddr, toName, toAddr, subject, content)

	err := smtp.SendMail(emailer.address, emailer.auth, fromAddr, toList, []byte(data))
	return err
}

func SendTextEmail(fromAddr, fromName, toAddr, toName, subject, content string) error {
	toList := []string{toAddr}

	data := fmt.Sprintf(`From: %s <%s>
To: %s <%s>
Subject: %s
MIME-version: 1.0;
Content-Type: text/plain; charset="utf-8";

%s
`, fromName, fromAddr, toName, toAddr, subject, content)

	err := smtp.SendMail(emailer.address, emailer.auth, fromAddr, toList, []byte(data))
	return err
}

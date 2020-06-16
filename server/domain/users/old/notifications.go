package users

import (
	"bytes"
	"fmt"
	"net/mail"
	"text/template"
	"time"

	"gitlab.com/bloom42/bloom/server/services/notification"
)

const userVerificationEmailTemplate = `
Welcome aboard {{ .Name }}! <br/>
Your confirmation code is: <br/>
<h2>{{ .FormattedCode }}</h2> <br/>
This code will only be valid for 30 minutes. <br/>
If you did not ask for a code, please ignore this email.
`

type userVerificationEmailData struct {
	Name          string
	FormattedCode string
}

func sendUserVerificationCode(toAddr, displayName, code string) error {
	var content bytes.Buffer
	tmpl := template.Must(template.New("userVerificationEmailTemplate").Parse(userVerificationEmailTemplate))
	formattedCode := code[:4] + "-" + code[4:]
	data := userVerificationEmailData{
		FormattedCode: formattedCode,
		Name:          displayName,
	}

	subject := fmt.Sprintf("Bloom confirmation code: %s", formattedCode)
	err := tmpl.Execute(&content, data)
	if err != nil {
		return err
	}

	from := &mail.Address{Name: notification.DEFAULT_SENDER_NAME, Address: notification.DEFAULT_SENDER_ADDRESS}
	to := []*mail.Address{{Name: displayName, Address: toAddr}}
	return notification.SendHTMLEmailWithDefaultTemplate(from, to, subject, content.Bytes())
}

const userSignInAlertTemplate = `
Hi {{ .Name }}! <br/><br/>

A new device connected to your account on {{ .Time }} from the IP {{ .IP }} <br/><br/>

If it's not you, please change your password immediatly, otherwise you can safely ignore this email.
`

type userSignInAlertData struct {
	Name string
	IP   string
	Time string
}

func sendSignInEmailAlert(toAddr, toName, ipAddress string) error {
	var content bytes.Buffer
	tmpl := template.Must(template.New("userSignInAlertTemplate").Parse(userSignInAlertTemplate))
	now := time.Now().UTC()
	data := userSignInAlertData{
		Name: toName,
		IP:   ipAddress,
		Time: now.Format(time.RFC3339),
	}

	subject := "Bloom - Sign-in alert"
	err := tmpl.Execute(&content, data)
	if err != nil {
		return err
	}

	from := &mail.Address{Name: notification.DEFAULT_SENDER_NAME, Address: notification.DEFAULT_SENDER_ADDRESS}
	to := []*mail.Address{{Name: toName, Address: toAddr}}
	return notification.SendHTMLEmailWithDefaultTemplate(from, to, subject, content.Bytes())
}

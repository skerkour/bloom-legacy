package users

import (
	"bytes"
	"fmt"
	"gitlab.com/bloom42/bloom/server/services/notification"
	"text/template"
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

func SendUserVerificationCode(toAddr, displayName, code string) error {
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

	err = notification.SendHTMLEmail("hello@bloom.sh", "Bloom", toAddr, displayName, subject, content.String())
	return err
}

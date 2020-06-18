package service

import (
	"bytes"
	"context"
	"net/mail"

	"gitlab.com/bloom42/bloom/server/app/config"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/email"
	"gitlab.com/bloom42/gobox/log"
)

func (service *UsersService) sendRegisterEmail(ctx context.Context, code string, pendingUser users.PendingUser) (err error) {
	var content bytes.Buffer
	logger := log.FromCtx(ctx)
	to := mail.Address{
		Name:    pendingUser.DisplayName,
		Address: pendingUser.Email,
	}
	subject := "Bloom code: " + code
	htmlCode := confirmationCodeToHTML(code)
	params := registerEmailParams{
		Code: htmlCode,
	}

	err = service.registerEmailTemplate.Execute(&content, params)
	if err != nil {
		errMessage := "users.sendRegisterEmail: Executing email template"
		logger.Error(errMessage, log.Err("err", err))
		err = errors.Internal(errMessage, err)
		return
	}

	message := email.Email{
		From:    config.DefaultEmailAddressFrom,
		To:      []mail.Address{to},
		Subject: subject,
		HTML:    content.Bytes(),
		Text:    content.Bytes(),
	}
	err = service.mailer.Send(ctx, message)
	if err != nil {
		errMessage := "users.sendRegisterEmail: Sending email"
		logger.Error(errMessage, log.Err("err", err))
		err = errors.Internal(errMessage, err)
	}
	return
}

type registerEmailParams struct {
	Code string
}

const registerEmailTemplate = `
<h1> {{ .Code }} </h1>
`

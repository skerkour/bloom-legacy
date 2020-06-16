package service

import (
	"text/template"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/driver"
)

// UsersService is an implementation of `users.Service`
type UsersService struct {
	usersRepo             users.Repository
	bilingService         billing.Service
	db                    db.DB
	mailer                driver.Mailer
	signInEmailTemplate   *template.Template
	registerEmailTemplate *template.Template
}

// NewUsersService instantiate a new `UsersService`
func NewUsersService(db db.DB, userRepo users.Repository, mailer driver.Mailer) *UsersService {
	signInEmailTemplate := template.Must(template.New("signInEmailTemplate").Parse(signInEmailTemplate))
	registerEmailTemplate := template.Must(template.New("registerEmailTemplate").Parse(registerEmailTemplate))

	return &UsersService{
		userRepo:              userRepo,
		db:                    db,
		mailer:                mailer,
		signInEmailTemplate:   signInEmailTemplate,
		registerEmailTemplate: registerEmailTemplate,
	}
}

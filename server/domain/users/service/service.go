package service

import (
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/driver"
)

// UsersService is an implementation of `users.Service`
type UsersService struct {
	usersRepo     users.Repository
	bilingService billing.Service
	db            db.DB
	mailer        driver.Mailer
}

// NewUsersService instantiate a new `UsersService`
func NewUsersService(db db.DB, usersRepo users.Repository, billingService billing.Service, mailer driver.Mailer) *UsersService {
	return &UsersService{
		usersRepo:     usersRepo,
		db:            db,
		mailer:        mailer,
		bilingService: billingService,
	}
}

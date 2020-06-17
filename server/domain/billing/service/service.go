package service

import (
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/driver"
)

// BillingService is an implementation of `billing.Service`
type BillingService struct {
	db            db.DB
	billingRepo   billing.Repository
	usersService  users.Service
	groupsService groups.Service
	mailer        driver.Mailer
}

// NewBillingService instantiate a new `BillingService`
func NewBillingService(db db.DB, billingRepo billing.Repository, mailer driver.Mailer) *BillingService {
	return &BillingService{
		db:          db,
		billingRepo: billingRepo,
		mailer:      mailer,
	}
}

func (service *BillingService) InjectUsersService(usersService users.Service) {
	service.usersService = usersService
}

func (service *BillingService) InjectGroupsService(groupsService groups.Service) {
	service.groupsService = groupsService
}

package service

import (
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/domain/users"
)

// BillingService is an implementation of `billing.Service`
type BillingService struct {
	db            db.DB
	billingRepo   billing.Repository
	usersService  users.Service
	groupsService groups.Service
}

// NewBillingService instantiate a new `BillingService`
func NewBillingService(db db.DB, billingRepo billing.Repository) *BillingService {
	return &BillingService{
		db:          db,
		billingRepo: billingRepo,
	}
}

func (service *BillingService) InjectUsersService(usersService users.Service) {
	service.usersService = usersService
}

func (service *BillingService) InjectGroupsService(groupsService groups.Service) {
	service.groupsService = groupsService
}

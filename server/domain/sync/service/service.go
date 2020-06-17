package service

import (
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/domain/sync"
	"gitlab.com/bloom42/bloom/server/domain/users"
)

// SyncService is an implementation of `sync.Service`
type SyncService struct {
	db             db.DB
	usersService   users.Service
	groupsService  groups.Service
	syncRepo       sync.Repository
	groupsRepo     groups.Repository
	billingRepo    billing.Repository
	billingService billing.Service
	usersRepo      users.Repository
}

// NewSyncService instantiate a new `BillingService`
func NewSyncService(db db.DB, usersService users.Service, groupsService groups.Service, syncRepo sync.Repository,
	groupsRepo groups.Repository, billingRepo billing.Repository, billingService billing.Service,
	usersRepo users.Repository) *SyncService {
	return &SyncService{
		db:             db,
		usersService:   usersService,
		groupsService:  groupsService,
		syncRepo:       syncRepo,
		groupsRepo:     groupsRepo,
		billingRepo:    billingRepo,
		billingService: billingService,
		usersRepo:      usersRepo,
	}
}

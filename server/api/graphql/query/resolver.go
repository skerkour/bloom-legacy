package query

import (
	"gitlab.com/bloom42/bloom/server/app/config"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/domain/sync"
	"gitlab.com/bloom42/bloom/server/domain/users"
)

// Resolver is the QueryResolver
type Resolver struct {
	usersService   users.Service
	syncService    sync.Service
	groupsService  groups.Service
	billingService billing.Service
	config         config.Config
}

// NewResolver resturns a new Resolver with the appropriate dependencies
func NewResolver(conf config.Config, usersService users.Service, groupsService groups.Service,
	syncService sync.Service, billingService billing.Service) *Resolver {
	return &Resolver{
		config:         conf,
		usersService:   usersService,
		syncService:    syncService,
		groupsService:  groupsService,
		billingService: billingService,
	}
}

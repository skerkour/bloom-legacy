package mutation

import (
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/domain/sync"
	"gitlab.com/bloom42/bloom/server/domain/users"
)

// Resolver is the MutationResolver
type Resolver struct {
	usersService   users.Service
	groupsService  groups.Service
	syncService    sync.Service
	billingService billing.Service
}

// NewResolver resturns a new Resolver with the appropriate dependencies
func NewResolver(usersService users.Service, groupsService groups.Service, syncService sync.Service, billingService billing.Service) *Resolver {
	return &Resolver{
		usersService:   usersService,
		groupsService:  groupsService,
		syncService:    syncService,
		billingService: billingService,
	}
}

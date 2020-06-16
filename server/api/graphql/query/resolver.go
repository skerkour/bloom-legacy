package query

import (
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/domain/sync"
	"gitlab.com/bloom42/bloom/server/domain/users"
)

// Resolver is the QueryResolver
type Resolver struct {
	usersService  users.Service
	syncService   sync.Service
	groupsService groups.Service
}

// NewResolver resturns a new Resolver with the appropriate dependencies
func NewResolver(usersService users.Service, groupsService groups.Service, syncService sync.Service) *Resolver {
	return &Resolver{
		usersService:  usersService,
		syncService:   syncService,
		groupsService: groupsService,
	}
}

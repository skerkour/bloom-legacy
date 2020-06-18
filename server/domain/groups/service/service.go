package service

import (
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/domain/users"
)

// GroupsService is an implementation of `groups.Service`
type GroupsService struct {
	db             db.DB
	groupsRepo     groups.Repository
	usersService   users.Service
	billingService billing.Service
}

// NewGroupsService instantiate a new `GroupsService`
func NewGroupsService(db db.DB, groupsRepo groups.Repository, usersService users.Service, billingService billing.Service) *GroupsService {
	return &GroupsService{
		db:             db,
		groupsRepo:     groupsRepo,
		usersService:   usersService,
		billingService: billingService,
	}
}

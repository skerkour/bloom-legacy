package service

import (
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/groups"
)

// GroupsService is an implementation of `groups.Service`
type GroupsService struct {
	db         db.DB
	groupsRepo groups.Repository
}

// NewGroupsService instantiate a new `GroupsService`
func NewGroupsService(db db.DB, groupsRepo groups.Repository) *GroupsService {
	return &GroupsService{
		db:         db,
		groupsRepo: groupsRepo,
	}
}

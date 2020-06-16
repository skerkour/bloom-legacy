package service

import (
	"gitlab.com/bloom42/bloom/server/db"
)

// GroupsService is an implementation of `groups.Service`
type GroupsService struct {
	db            db.DB
}

// NewGroupsService instantiate a new `GroupsService`
func NewGroupsService(db db.DB) *GroupsService {
	return &GroupsService{
		db:            db,
	}
}

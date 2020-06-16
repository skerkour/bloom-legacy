package service

import (
	"gitlab.com/bloom42/bloom/server/db"
)

// SyncService is an implementation of `sync.Service`
type SyncService struct {
	db db.DB
}

// NewSyncService instantiate a new `BillingService`
func NewSyncService(db db.DB) *SyncService {
	return &SyncService{
		db: db,
	}
}

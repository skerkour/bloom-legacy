package service

import (
	"gitlab.com/bloom42/bloom/server/db"
)

// BillingService is an implementation of `billing.Service`
type BillingService struct {
	db db.DB
}

// NewBillingService instantiate a new `BillingService`
func NewBillingService(db db.DB) *BillingService {
	return &BillingService{
		db: db,
	}
}

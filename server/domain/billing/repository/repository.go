package repository

// BillingRepository is an implementation of `billing.Repository`
type BillingRepository struct {
}

// NewBillingRepository returns a new instance of BillingRepository
func NewBillingRepository() *BillingRepository {
	return &BillingRepository{}
}

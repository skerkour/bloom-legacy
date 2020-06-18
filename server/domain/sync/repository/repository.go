package repository

// SyncRepository is an implementation of `sync.Repository`
type SyncRepository struct {
}

// NewSyncRepository returns a new instance of SyncRepository
func NewSyncRepository() *SyncRepository {
	return &SyncRepository{}
}

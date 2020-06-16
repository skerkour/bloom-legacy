package groups

import "context"

// Service is the application service for the groups subdomain
type Service interface {
	// Commands
	// Queries
	FindAllGroups(ctx context.Context) ([]Group, error)
}

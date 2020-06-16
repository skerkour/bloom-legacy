package groups

import (
	"context"

	"gitlab.com/bloom42/gobox/uuid"
)

// Service is the application service for the groups subdomain
type Service interface {
	// Commands
	// Queries
	FindAllGroups(ctx context.Context) ([]Group, error)
	FindGroupById(ctx context.Context, groupID uuid.UUID) (Group, error)
}

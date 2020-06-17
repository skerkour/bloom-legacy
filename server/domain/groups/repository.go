package groups

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/gobox/uuid"
)

// Repository is the repository for all the entities of the groups subdomain
type Repository interface {
	UpdateGroup(ctx context.Context, db db.Queryer, group Group) (err error)

	FindGroupByID(ctx context.Context, db db.Queryer, groupID uuid.UUID) (ret Group, err error)
}

package sync

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/gobox/uuid"
)

// Repository is the repository for all the entities of the sync subdomain
type Repository interface {
	DeleteGroupObjects(ctx context.Context, db db.Queryer, groupID uuid.UUID) (err error)
	FindObjectByID(ctx context.Context, db db.Queryer, objectID []byte, forUpdate bool) (ret Object, err error)
	FindObjectsSinceState(ctx context.Context, db db.Queryer, sinceState int64, userID, groupID *uuid.UUID) ([]Object, error)
}

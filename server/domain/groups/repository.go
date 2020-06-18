package groups

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/gobox/uuid"
)

// Repository is the repository for all the entities of the groups subdomain
type Repository interface {
	UpdateGroup(ctx context.Context, db db.Queryer, group Group) (err error)
	CreateMembership(ctx context.Context, db db.Queryer, membership Membership) (err error)
	DeleteInvitation(ctx context.Context, db db.Queryer, invitationID uuid.UUID) (err error)

	FindGroupByID(ctx context.Context, db db.Queryer, groupID uuid.UUID) (ret Group, err error)
	FindInvitationByID(ctx context.Context, db db.Queryer, invitationID uuid.UUID) (ret Invitation, err error)
}

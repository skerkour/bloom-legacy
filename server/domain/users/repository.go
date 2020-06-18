package users

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/gobox/uuid"
)

// Repository is the repository for all the entities of the users subdomain
type Repository interface {
	// User
	CreateUser(ctx context.Context, db db.Queryer, user User) error
	UpdateUser(ctx context.Context, db db.Queryer, user User) error
	DeleteUser(ctx context.Context, db db.Queryer, userID uuid.UUID) error
	FindUserByID(ctx context.Context, db db.Queryer, userID uuid.UUID) (User, error)
	FindUserByEmail(ctx context.Context, db db.Queryer, email string) (User, error)
	FindUserByUsername(ctx context.Context, db db.Queryer, username string) (User, error)

	// Session
	CreateSession(ctx context.Context, db db.Queryer, session Session) error
	DeleteSession(ctx context.Context, db db.Queryer, sessionID uuid.UUID) error
	FindSessionByID(ctx context.Context, db db.Queryer, sessionID uuid.UUID) (Session, error)
	DeleteAllSessionsForUser(ctx context.Context, db db.Queryer, userID uuid.UUID) (err error)
	FindAllSessionsForUser(ctx context.Context, db db.Queryer, userID uuid.UUID) (ret []Session, err error)

	// PendingUser
	FindPendingUserByID(ctx context.Context, db db.Queryer, pendingUserID uuid.UUID) (PendingUser, error)
	CreatePendingUser(ctx context.Context, db db.Queryer, pendingUser PendingUser) error
	DeletePendingUser(ctx context.Context, db db.Queryer, pendingUserID uuid.UUID) error
	UpdatePendingUser(ctx context.Context, db db.Queryer, pendingUser PendingUser) error

	// PendingSession
	FindPendingSessionByID(ctx context.Context, db db.Queryer, pendingSessionID uuid.UUID) (PendingSession, error)
	DeletePendingSession(ctx context.Context, db db.Queryer, pendingSessionID uuid.UUID) error
	UpdatePendingSession(ctx context.Context, db db.Queryer, pendingSession PendingSession) error
}

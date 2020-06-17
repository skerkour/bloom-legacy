package service

import (
	"context"

	"gitlab.com/bloom42/gobox/uuid"
)

func (service *UsersService) RevokeSession(ctx context.Context, sessionID uuid.UUID) (err error) {
	return
}

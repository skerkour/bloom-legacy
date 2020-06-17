package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *UsersService) StartRegistration(ctx context.Context, params users.StartRegistrationParams) (newPendingUserID uuid.UUID, err error) {
	return
}

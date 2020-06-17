package service

import (
	"context"

	"gitlab.com/bloom42/gobox/uuid"
)

func (service *UsersService) SendNewRegistrationCode(ctx context.Context, pendingUserID uuid.UUID) (err error) {
	return
}

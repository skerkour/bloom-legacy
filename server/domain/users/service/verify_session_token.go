package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/users"
)

func (service *UsersService) VerifySessionToken(ctx context.Context, token string) (user users.User, session users.Session, err error) {
	return
}

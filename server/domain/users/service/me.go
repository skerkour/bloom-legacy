package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/http/httputil"
)

func (service *UsersService) Me(ctx context.Context) (me users.User, err error) {
	httpCtx := httputil.HTTPCtxFromCtx(ctx)

	if httpCtx.AuthenticatedUser == nil {
		err = users.ErrAuthenticationRequired
		return
	}

	me = *httpCtx.AuthenticatedUser
	return
}

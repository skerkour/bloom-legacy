package apiutil

import (
	"context"

	"gitlab.com/bloom42/libs/rz-go"
	"gitlab.com/bloom42/bloom/server/api/apictx"
	"gitlab.com/bloom42/bloom/server/domain/users"
)

func UserFromCtx(ctx context.Context) *users.User {
	apiCtx, ok := ctx.Value(apictx.Key).(*apictx.Context)
	if !ok {
		logger := rz.FromCtx(ctx)
		logger.Error("gqlutil.UserFromCtx: error getting apiCtx from context")
		return nil
	}
	return apiCtx.AuthenticatedUser
}

func ApiCtxFromCtx(ctx context.Context) *apictx.Context {
	apiCtx, ok := ctx.Value(apictx.Key).(*apictx.Context)
	if !ok {
		logger := rz.FromCtx(ctx)
		logger.Error("gqlutil.UserFromCtx: error getting apiCtx from context")
		return nil
	}
	return apiCtx
}

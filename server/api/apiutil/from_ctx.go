package apiutil

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api/apictx"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/rz-go"
)

func UserFromCtx(ctx context.Context) *users.User {
	apiCtx, ok := ctx.Value(apictx.Key).(*apictx.Context)
	if !ok {
		logger := rz.FromCtx(ctx)
		logger.Error("apiutil.UserFromCtx: error getting apiCtx from context")
		return nil
	}
	return apiCtx.AuthenticatedUser
}

func SessionFromCtx(ctx context.Context) *users.Session {
	apiCtx, ok := ctx.Value(apictx.Key).(*apictx.Context)
	if !ok {
		logger := rz.FromCtx(ctx)
		logger.Error("apiutil.UserFromCtx: error getting apiCtx from context")
		return nil
	}
	return apiCtx.Session
}

func ApiCtxFromCtx(ctx context.Context) *apictx.Context {
	apiCtx, ok := ctx.Value(apictx.Key).(*apictx.Context)
	if !ok {
		logger := rz.FromCtx(ctx)
		logger.Error("apiutil.ApiCtxFromCtx: error getting apiCtx from context")
		return nil
	}
	return apiCtx
}

package httputil

import (
	"context"

	"gitlab.com/bloom42/bloom/server/http/httpctx"
	"gitlab.com/bloom42/gobox/log"
)

func HTTPCtxFromCtx(ctx context.Context) *httpctx.Context {
	httpCtx, ok := ctx.Value(httpctx.Key).(*httpctx.Context)
	if !ok {
		logger := log.FromCtx(ctx)
		logger.Error("httputil.HTTPContextFromCtx: error getting httpCtx from context")
		return nil
	}
	return httpCtx
}

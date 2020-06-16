package query

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/app"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/bloom/server/http/httputil"
	"gitlab.com/bloom42/bloom/server/server/api/graphql/model"
)

// Metadata returns infrmation about the Bloom server
func (resolver *Resolver) Metadata(ctx context.Context) (ret *model.BloomMetadata, err error) {
	httpCtx := httputil.HTTPCtxFromCtx(ctx)

	if httpCtx.AuthenticatedUser == nil {
		return "", api.NewError(errors.PermissionDenied("Authentication required."))
	}

	ret = &model.BloomMetadata{
		Os:        app.OS,
		Arch:      app.Arch,
		Version:   app.Version,
		GitCommit: app.GitCommit,
	}
	return
}

package query

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/app"
	"gitlab.com/bloom42/bloom/server/errors"
)

// Metadata returns infrmation about the Bloom server
func (resolver *Resolver) Metadata(ctx context.Context) (ret *model.BloomMetadata, err error) {
	me, err := resolver.usersService.Me(ctx)
	if err != nil || !me.IsAdmin {
		return ret, api.NewError(errors.PermissionDenied("Authentication required."))
	}

	ret = &model.BloomMetadata{
		Os:        app.OS,
		Arch:      app.Arch,
		Version:   app.Version,
		GitCommit: app.GitCommit,
	}
	return
}

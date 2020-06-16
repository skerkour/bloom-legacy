package query

import (
	"context"

	"gitlab.com/bloom42/bloom/server/app"
	"gitlab.com/bloom42/bloom/server/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/server/api/graphql/model"
)

// Metadata returns infrmation about the Bloom server
func (resolver *Resolver) Metadata(ctx context.Context) (ret *model.BloomMetadata, err error) {
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil || !currentUser.IsAdmin {
		err = gqlerrors.AdminRoleRequired()
		return
	}

	ret = &model.BloomMetadata{
		Os:        app.OS,
		Arch:      app.Arch,
		Version:   app.Version,
		GitCommit: app.GitCommit,
	}
	return
}

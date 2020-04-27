package query

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/cmd/bloom/version"
)

// Metadata returns infrmation about the Bloom server
func (resolver *Resolver) Metadata(ctx context.Context) (ret *model.BloomMetadata, err error) {
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil || !currentUser.IsAdmin {
		err = gqlerrors.AdminRoleRequired()
		return
	}

	ret = &model.BloomMetadata{
		Os:        version.OS,
		Arch:      version.Arch,
		Version:   version.Version,
		GitCommit: version.GitCommit,
	}
	return
}

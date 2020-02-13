package query

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/version"
)

// Metadata returns infrmation about the Bloom server
func (resolver *Resolver) Metadata(ctx context.Context) (*model.BloomMetadata, error) {
	var ret *model.BloomMetadata
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil || !currentUser.IsAdmin {
		return ret, gqlerrors.AdminRoleRequired()
	}

	ret = &model.BloomMetadata{
		Os:        version.OS,
		Arch:      version.Arch,
		Version:   version.Version,
		GitCommit: version.GitCommit,
	}

	return ret, nil
}

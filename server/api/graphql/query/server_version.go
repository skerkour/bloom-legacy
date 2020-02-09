package query

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/version"
)

func (resolver *Resolver) ServerVersion(ctx context.Context) (*model.ServerVersion, error) {
	var ret *model.ServerVersion
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil || !currentUser.IsAdmin {
		return ret, gqlerrors.AdminRoleRequired()
	}

	ret = &model.ServerVersion{
		Os:        version.OS,
		Arch:      version.Arch,
		Version:   version.Version,
		GitCommit: version.GitCommit,
	}

	return ret, nil
}

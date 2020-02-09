package query

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/version"
)

func (resolver *Resolver) ServerVersion(ctx context.Context) (*model.SeverVerion, error) {
	var ret *model.SeverVerion
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil || !currentUser.IsAdmin {
		return ret, gqlerrors.AdminRoleRequired()
	}

	ret = &model.SeverVerion{
		Os:        version.OS,
		Arch:      version.Arch,
		Version:   version.Version,
		GitCommit: version.GitCommit,
	}

	return ret, nil
}

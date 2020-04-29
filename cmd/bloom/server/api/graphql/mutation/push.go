package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/sync"
	"gitlab.com/bloom42/bloom/server/api/graphql/gqlerrors"
)

// Push is used to push changes
func (resolver *Resolver) Push(ctx context.Context, input model.PushInput) (ret *model.Push, err error) {
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		err = gqlerrors.AuthenticationRequired()
		return
	}

	repositories := []sync.RepositoryPush{}
	for _, repo := range input.Repositories {
		repositoryPush := sync.RepositoryPush{
			CurrentState: repo.CurrentState,
			GroupID:      repo.GroupID,
			Objects:      []sync.APIObject{},
		}
		for _, object := range repo.Objects {
			obj := sync.APIObject{
				ID:            object.ID,
				Algorithm:     object.Algorithm,
				EncryptedData: object.EncryptedData,
				EncryptedKey:  object.EncryptedKey,
				Nonce:         object.Nonce,
			}
			repositoryPush.Objects = append(repositoryPush.Objects, obj)
		}
		repositories = append(repositories, repositoryPush)
	}

	params := sync.PushParams{Repositories: repositories}
	result, err := sync.Push(ctx, currentUser, params)
	if err != nil {
		err = gqlerrors.New(err)
		return
	}

	ret = &model.Push{Repositories: []*model.RepositoryPush{}}
	for _, push := range result.Repositories {
		resPush := &model.RepositoryPush{
			OldState: push.OldState,
			NewState: push.NewState,
			GroupID:  push.GroupID,
		}
		ret.Repositories = append(ret.Repositories, resPush)
	}

	return
}

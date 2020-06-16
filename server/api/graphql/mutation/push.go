package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/server/domain/objects"
)

// Push is used to push changes
func (resolver *Resolver) Push(ctx context.Context, input model.PushInput) (ret *model.Push, err error) {
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		err = gqlerrors.AuthenticationRequired()
		return
	}

	repositories := []objects.RepositoryPush{}
	for _, repo := range input.Repositories {
		repositoryPush := objects.RepositoryPush{
			CurrentState: repo.CurrentState,
			GroupID:      repo.GroupID,
			Objects:      []objects.APIObject{},
		}
		for _, object := range repo.Objects {
			obj := objects.APIObject{
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

	params := objects.PushParams{Repositories: repositories}
	result, err := objects.Push(ctx, currentUser, params)
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

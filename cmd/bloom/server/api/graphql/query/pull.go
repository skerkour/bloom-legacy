package query

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/sync"
)

// Pull returns the changes from a given state
func (resolver *Resolver) Pull(ctx context.Context, input model.PullInput) (ret *model.Pull, err error) {
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		err = gqlerrors.AuthenticationRequired()
		return
	}

	repositories := []sync.RepositoryPull{}
	for _, repo := range input.Repositories {
		repository := sync.RepositoryPull{
			SinceState: repo.SinceState,
			GroupID:    repo.GroupID,
		}
		repositories = append(repositories, repository)
	}

	params := sync.PullParams{Repositories: repositories}
	result, err := sync.Pull(ctx, currentUser, params)
	if err != nil {
		err = gqlerrors.New(err)
		return
	}

	ret = &model.Pull{Repositories: []*model.RepositoryPull{}}
	for _, repo := range result.Repositories {
		repository := &model.RepositoryPull{
			OldState:       repo.OldState,
			NewState:       repo.NewState,
			HasMoreChanges: repo.HasMoreChanges,
			GroupID:        repo.GroupID,
			Objects:        []*model.Object{},
		}
		for _, object := range repo.Objects {
			obj := &model.Object{
				ID:            object.ID,
				Algorithm:     object.Algorithm,
				EncryptedData: object.EncryptedData,
				EncryptedKey:  object.EncryptedKey,
				Nonce:         object.Nonce,
			}
			repository.Objects = append(repository.Objects, obj)
		}
		ret.Repositories = append(ret.Repositories, repository)
	}
	return
}

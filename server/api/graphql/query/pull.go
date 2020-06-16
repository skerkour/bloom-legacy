package query

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/server/domain/objects"
)

// Pull returns the changes from a given state
func (resolver *Resolver) Pull(ctx context.Context, input model.PullInput) (ret *model.Pull, err error) {
	repositories := []objects.RepositoryPull{}
	for _, repo := range input.Repositories {
		repository := objects.RepositoryPull{
			SinceState: repo.SinceState,
			GroupID:    repo.GroupID,
		}
		repositories = append(repositories, repository)
	}

	params := objects.PullParams{
		Repositories: repositories,
	}
	result, err := resolver.syncService.Pull(ctx, params)
	if err != nil {
		err = api.NewError(err)
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

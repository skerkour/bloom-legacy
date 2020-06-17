package service

import (
	"context"

	"github.com/gofrs/uuid"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/domain/sync"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
)

func (service *SyncService) Pull(ctx context.Context, params sync.PullParams) (ret sync.PullResult, err error) {
	ret = PullResult{
		Repositories: []RepositoryPullResult{},
	}
	me, err := service.usersService.Me(ctx)
	if err != nil {
		return
	}
	logger := log.FromCtx(ctx)

	// clean and validate params
	for i, repo := range params.Repositories {
		var sinceStateInt int64
		sinceStateInt, err = sync.DecodeStateString(repo.SinceState)
		if err != nil {
			logger.Warn("sync.Pull: PullError decoding state",
				log.String("state", repo.SinceState), log.UUID("user.id", me.ID))
			return
		}
		params.Repositories[i].sinceStateInt = sinceStateInt
	}

	groups, err := service.groupsService.FindGroupsForUser(ctx, me.ID)
	if err != nil {
		return
	}

	params.Repositories = cleanPulls(ctx, groups, params.Repositories)

	tx, err := service.db.Begin(ctx)
	if err != nil {
		errMessage := "sync.Pull: starting transaction"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}

	for _, repo := range params.Repositories {
		var result RepositoryPullResult
		result, err = service.pullRepository(ctx, tx, me, repo)
		if err != nil {
			tx.Rollback()
			return
		}
		ret.Repositories = append(ret.Repositories, result)
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		errMessage := "sync.Pull: committing transaction"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}
	return
}

func (service *SyncService) pullRepository(ctx context.Context, db db.Queryer, actor users.User, repo RepositoryPull) (ret RepositoryPullResult, err error) {
	ret.Objects = []Object{}
	ret.OldState = repo.SinceState

	if repo.group != nil {
		var objects []Object

		if repo.sinceStateInt == repo.group.State {
			ret.NewState = sync.EncodeState(repo.group.State)
			return
		}
		objects, err = service.syncRepo.FindObjectsSinceState(ctx, db, repo.sinceStateInt, nil, &repo.group.ID)
		if err != nil {
			return
		}
		ret.NewState = sync.EncodeState(repo.group.State)
		ret.Objects = objects

	} else {
		// user's repository
		var objects []Object

		if repo.sinceStateInt == actor.State {
			ret.NewState = sync.EncodeState(actor.State)
			return
		}

		objects, err = FindObjectSinceState(ctx, db, repo.sinceStateInt, &actor.ID, nil)
		if err != nil {
			return
		}
		ret.NewState = service.syncRepo.EncodeState(actor.State)
		ret.Objects = objects
	}
	return
}

func cleanPulls(ctx context.Context, userGroups []groups.Group, pulls []RepositoryPull) []RepositoryPull {
	ret := []RepositoryPull{}
	nullGroupPassed := false
	groupsSet := map[uuid.UUID]groups.Group{}

	for _, group := range userGroups {
		groupsSet[group.ID] = group
	}

	// remove duplicates and groups where user is not in
	for _, pull := range pulls {
		if pull.GroupID != nil {
			group, inUserGroups := groupsSet[*pull.GroupID]
			if inUserGroups {
				delete(groupsSet, *pull.GroupID)
				pull.group = &group
				ret = append(ret, pull)
			}
		} else {
			if !nullGroupPassed {
				nullGroupPassed = true
				ret = append(ret, pull)
			}
		}
	}

	// all remaining groups
	for _, group := range groupsSet {
		pull := RepositoryPull{
			GroupID:       &group.ID,
			sinceStateInt: 0,
			SinceState:    "",
			group:         &group,
		}
		ret = append(ret, pull)
	}

	if !nullGroupPassed {
		pullMe := RepositoryPull{
			sinceStateInt: 0,
			SinceState:    "",
		}
		ret = append(ret, pullMe)
	}
	return ret
}

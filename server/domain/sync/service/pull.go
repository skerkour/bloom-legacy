package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/domain/sync"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *SyncService) Pull(ctx context.Context, params sync.PullParams) (ret sync.PullResult, err error) {
	ret = sync.PullResult{
		Repositories: []sync.RepositoryPullResult{},
	}
	me, err := service.usersService.Me(ctx)
	if err != nil {
		return
	}
	logger := log.FromCtx(ctx)

	// clean and validate params
	for i, repo := range params.Repositories {
		var SinceStateInt int64
		SinceStateInt, err = sync.DecodeStateString(repo.SinceState)
		if err != nil {
			logger.Warn("sync.Pull: PullError decoding state",
				log.String("state", repo.SinceState), log.UUID("user.id", me.ID))
			return
		}
		params.Repositories[i].SinceStateInt = SinceStateInt
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
		var result sync.RepositoryPullResult
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

func (service *SyncService) pullRepository(ctx context.Context, db db.Queryer, actor users.User, repo sync.RepositoryPull) (ret sync.RepositoryPullResult, err error) {
	ret.Objects = []sync.Object{}
	ret.OldState = repo.SinceState

	if repo.Group != nil {
		var objects []sync.Object

		if repo.SinceStateInt == repo.Group.State {
			ret.NewState = sync.EncodeState(repo.Group.State)
			return
		}
		objects, err = service.syncRepo.FindObjectsSinceState(ctx, db, repo.SinceStateInt, nil, &repo.Group.ID)
		if err != nil {
			return
		}
		ret.NewState = sync.EncodeState(repo.Group.State)
		ret.Objects = objects

	} else {
		// user's repository
		var objects []sync.Object

		if repo.SinceStateInt == actor.State {
			ret.NewState = sync.EncodeState(actor.State)
			return
		}

		objects, err = service.syncRepo.FindObjectsSinceState(ctx, db, repo.SinceStateInt, &actor.ID, nil)
		if err != nil {
			return
		}
		ret.NewState = sync.EncodeState(actor.State)
		ret.Objects = objects
	}
	return
}

func cleanPulls(ctx context.Context, userGroups []groups.Group, pulls []sync.RepositoryPull) []sync.RepositoryPull {
	ret := []sync.RepositoryPull{}
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
				pull.Group = &group
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
		pull := sync.RepositoryPull{
			GroupID:       &group.ID,
			SinceStateInt: 0,
			SinceState:    "",
			Group:         &group,
		}
		ret = append(ret, pull)
	}

	if !nullGroupPassed {
		pullMe := sync.RepositoryPull{
			SinceStateInt: 0,
			SinceState:    "",
		}
		ret = append(ret, pullMe)
	}
	return ret
}

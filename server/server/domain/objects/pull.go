package objects

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/server/server/db"
	"gitlab.com/bloom42/bloom/server/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/server/domain/users"
	"gitlab.com/bloom42/gobox/rz"
	"gitlab.com/bloom42/gobox/uuid"
)

// PullParams are the parameters for Pull
type PullParams struct {
	Repositories []RepositoryPull
}

type RepositoryPull struct {
	SinceState    string
	sinceStateInt int64
	GroupID       *uuid.UUID
	group         *groups.Group
}

type PullResult struct {
	Repositories []RepositoryPullResult
}

type RepositoryPullResult struct {
	OldState       string
	NewState       string
	HasMoreChanges bool
	Objects        []Object
	GroupID        *uuid.UUID
}

// Pull is used to pull changes
func Pull(ctx context.Context, actor *users.User, params PullParams) (ret *PullResult, err error) {
	ret = &PullResult{Repositories: []RepositoryPullResult{}}
	logger := rz.FromCtx(ctx)

	// clean and validate params
	for i, repo := range params.Repositories {
		var sinceStateInt int64
		sinceStateInt, err = DecodeStateString(repo.SinceState)
		if err != nil {
			logger.Warn("Error decoding state", rz.String("state", repo.SinceState), rz.String("user.id", actor.ID.String()))
			err = NewError(ErrorInternal)
			return
		}
		params.Repositories[i].sinceStateInt = sinceStateInt
	}

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("objects.Pull: Starting transaction", rz.Err(err))
		err = NewError(ErrorInternal)
		return
	}

	// fetch user's groups
	groups, err := groups.FindGroupsForUser(ctx, tx, actor.ID)
	if err != nil {
		tx.Rollback()
		return
	}

	params.Repositories = cleanPulls(ctx, groups, params.Repositories)

	for _, repo := range params.Repositories {
		var result RepositoryPullResult
		result, err = pullRepository(ctx, tx, actor, &repo)
		if err != nil {
			tx.Rollback()
			return
		}
		ret.Repositories = append(ret.Repositories, result)
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("objects.Pull: Committing transaction", rz.Err(err))
		err = NewError(ErrorInternal)
		return
	}
	return
}

func pullRepository(ctx context.Context, tx *sqlx.Tx, actor *users.User, repo *RepositoryPull) (ret RepositoryPullResult, err error) {
	ret.Objects = []Object{}
	ret.OldState = repo.SinceState

	if repo.group != nil {
		var objects []Object

		if repo.sinceStateInt == repo.group.State {
			ret.NewState = EncodeState(repo.group.State)
			return
		}
		objects, err = FindObjectSinceState(ctx, tx, repo.sinceStateInt, nil, &repo.group.ID)
		if err != nil {
			return
		}
		ret.NewState = EncodeState(repo.group.State)
		ret.Objects = objects

	} else {
		// user's repository
		var objects []Object

		if repo.sinceStateInt == actor.State {
			ret.NewState = EncodeState(actor.State)
			return
		}

		objects, err = FindObjectSinceState(ctx, tx, repo.sinceStateInt, &actor.ID, nil)
		if err != nil {
			return
		}
		ret.NewState = EncodeState(actor.State)
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

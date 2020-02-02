package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/rz-go"
)

func (r *Resolver) RevokeSession(ctx context.Context, input model.RevokeSessionInput) (bool, error) {
	ret := false
	logger := rz.FromCtx(ctx)
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("mutation.RevokeSession: Starting transaction", rz.Err(err))
		return ret, gqlerrors.New(users.NewError(users.ErrorDeletingSession))
	}

	err = users.DeleteSession(ctx, tx, input.ID, currentUser.ID)
	if err != nil {
		tx.Rollback()
		return ret, gqlerrors.New(err)
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("mutation.RevokeSession: committing transaction", rz.Err(err))
		return ret, gqlerrors.New(users.NewError(users.ErrorDeletingSession))
	}

	ret = true
	return ret, nil
}

package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/users"
)

func (service *UsersService) CompleteRegistration(ctx context.Context, params users.CompleteRegistrationParams) (user users.User, session users.Session, token string, err error) {
	return
}

/*

	logger := rz.FromCtx(ctx)
	currentUser := apiutil.UserFromCtx(ctx)
	apiCtx := apiutil.ApiCtxFromCtx(ctx)
	if apiCtx == nil {
		logger.Error("mutation.CompleteRegistration: error getting apiCtx from context")
		err = gqlerrors.Internal()
		return
	}

	if currentUser != nil {
		return ret, gqlerrors.MustNotBeAuthenticated()
	}

	// sleep to prevent spam and bruteforce
	sleep, err := crypto.RandInt64(500, 800)
	if err != nil {
		logger.Error("mutation.CompleteRegistration: generating random int", rz.Err(err))
		err = gqlerrors.New(users.NewError(users.ErrorCompletingRegistration))
		return
	}
	time.Sleep(time.Duration(sleep) * time.Millisecond)

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("mutation.CompleteRegistration: Starting transaction", rz.Err(err))
		err = gqlerrors.New(users.NewError(users.ErrorCompletingRegistration))
		return
	}


	// create customer profile
	_, err = billing.CreateCustomer(ctx, tx, newUser, &newUser.ID, nil)
	if err != nil {
		tx.Rollback()
		err = gqlerrors.New(err)
		return
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("mutation.CompleteRegistration: Committing transaction", rz.Err(err))
		err = gqlerrors.Internal()
		return
	}
*/

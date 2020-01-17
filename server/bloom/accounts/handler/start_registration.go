package handler

import (
	"context"

	"github.com/twitchtv/twirp"
	rpc "gitlab.com/bloom42/bloom/core/rpc/accounts"
	"gitlab.com/bloom42/bloom/server/bloom/accounts"
	"gitlab.com/bloom42/bloom/server/db"
)

func (s Handler) StartRegistration(ctx context.Context, params *rpc.StartRegistrationParams) (*rpc.RegistrationStarted, error) {
	tx, err := db.DB.Beginx()
	if err != nil {
		return nil, twirp.InternalError(err.Error())
	}
	newPendingAccount, err := accounts.CreatePendingAccount(tx, params.DisplayName, params.Email)
	err = tx.Commit()
	if err != nil {
		return nil, twirp.InternalError(err.Error())
	}

	ret := rpc.RegistrationStarted{
		Id: newPendingAccount.ID,
	}
	return &ret, nil
}

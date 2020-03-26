package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/bloom/server/db"
	"gitlab.com/bloom42/bloom/bloom/server/domain/groups"
	"gitlab.com/bloom42/libs/rz-go"
)

func (r *Resolver) AcceptGroupInvitation(ctx context.Context, input model.AcceptGroupInvitationInput) (*model.Group, error) {
	var ret *model.Group
	logger := rz.FromCtx(ctx)
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("Starting transaction", rz.Err(err))
		return ret, gqlerrors.New(groups.NewError(groups.ErrorAcceptingInvitation))
	}

	var invitation groups.Invitation

	queryGetInvitation := "SELECT * FROM groups_invitations WHERE id = $1 FOR UPDATE"
	err = tx.Get(&invitation, queryGetInvitation, input.ID)
	if err != nil {
		tx.Rollback()
		logger.Error("fetching invitation", rz.Err(err),
			rz.String("invitation_id", input.ID))
		return ret, gqlerrors.New(groups.NewError(groups.ErrorAcceptingInvitation))
	}

	group, err := groups.AcceptInvitation(ctx, tx, *currentUser, invitation)
	if err != nil {
		tx.Rollback()
		return ret, gqlerrors.New(err)
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("mutation.AcceptGroupInvitation: Committing transaction", rz.Err(err))
		return ret, gqlerrors.New(groups.NewError(groups.ErrorAcceptingInvitation))
	}

	ret = &model.Group{
		ID:          &group.ID,
		CreatedAt:   &group.CreatedAt,
		Name:        group.Name,
		Description: group.Description,
		AvatarURL:   nil,
	}
	return ret, nil
}

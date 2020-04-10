package query

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
)

// Me returns the current user
func (resolver *Resolver) Me(ctx context.Context) (*model.User, error) {
	var ret *model.User
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	encryptedPrivateKey := model.Bytes(currentUser.EncryptedPrivateKey)
	ret = &model.User{
		ID:                  &currentUser.ID,
		AvatarURL:           nil,
		CreatedAt:           &currentUser.CreatedAt,
		Username:            currentUser.Username,
		FirstName:           &currentUser.FirstName,
		LastName:            &currentUser.LastName,
		DisplayName:         currentUser.DisplayName,
		IsAdmin:             currentUser.IsAdmin,
		Bio:                 currentUser.Bio,
		Email:               &currentUser.Email,
		EncryptedPrivateKey: &encryptedPrivateKey,
		PublicKey:           model.Bytes(currentUser.PublicKey),
	}

	return ret, nil
}

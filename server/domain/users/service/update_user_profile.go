package service

import (
	"context"
	"strings"
	"time"

	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *UsersService) UpdateUserProfile(ctx context.Context, params users.UpdateUserProfileParams) (user users.User, err error) {
	me, err := service.Me(ctx)
	if err != nil {
		return
	}
	var userID uuid.UUID

	if params.UserID == nil {
		userID = me.ID
	} else {
		userID = *params.UserID
	}

	if userID != me.ID && !me.IsAdmin {
		err = users.ErrPermissionDenied
		return
	}

	user, err = service.usersRepo.FindUserByID(ctx, service.db, userID)
	if err != nil {
		return
	}

	if params.Bio != nil {
		*params.Bio = strings.TrimSpace(*params.Bio)
		err = validateBio(*params.Bio)
		if err != nil {
			return
		}
		user.Bio = *params.Bio
	}

	if params.DisplayName != nil {
		*params.DisplayName = strings.TrimSpace(*params.DisplayName)
		err = validateDisplayName(*params.DisplayName)
		if err != nil {
			return
		}
		user.DisplayName = *params.DisplayName
	}

	if params.FirstName != nil {
		*params.FirstName = strings.TrimSpace(*params.FirstName)
		err = validateFirstName(*params.FirstName)
		if err != nil {
			return
		}
		user.FirstName = *params.FirstName
	}

	if params.LastName != nil {
		*params.LastName = strings.TrimSpace(*params.LastName)
		err = validateLastName(*params.LastName)
		if err != nil {
			return
		}
		user.LastName = *params.LastName
	}

	user.UpdatedAt = time.Now().UTC()
	err = service.usersRepo.UpdateUser(ctx, service.db, user)
	return
}

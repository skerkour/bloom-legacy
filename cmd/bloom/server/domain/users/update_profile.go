package users

import (
	"context"
	"fmt"
	"strings"
	"time"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/db"
	"gitlab.com/bloom42/lily/rz"
	"gitlab.com/bloom42/lily/uuid"
)

// UpdateProfileParams are the parameters for UpdateProfile
type UpdateProfileParams struct {
	ID          *uuid.UUID `json:"id"`
	DisplayName *string    `json:"displayName"`
	Bio         *string    `json:"bio"`
	FirstName   *string    `json:"firstName"`
	LastName    *string    `json:"lastName"`
}

// UpdateProfile update a user's profile
func UpdateProfile(ctx context.Context, user *User, params UpdateProfileParams) (*User, error) {
	var err error
	logger := rz.FromCtx(ctx)
	var ret *User
	now := time.Now().UTC()

	// clean and validate params
	if user == nil {
		logger.Error("", rz.Err(NewError(ErrorUserIsNull)))
		return ret, NewError(ErrorUpdatingProfile)
	}

	if params.ID != nil && (user.ID != *params.ID || !user.IsAdmin) {
		return ret, NewError(ErrorUserNotFound)
	}

	if params.DisplayName == nil && params.FirstName == nil && params.Bio == nil && params.LastName == nil {
		return ret, NewError(ErrorAllFieldsAreEmpty)
	}

	if params.Bio != nil {
		*params.Bio = strings.TrimSpace(*params.Bio)
		err = ValidateBio(*params.Bio)
		if err != nil {
			err = NewErrorMessage(ErrorInvalidArgument, err.Error())
			return ret, err
		}
	}
	if params.DisplayName != nil {
		*params.DisplayName = strings.TrimSpace(*params.DisplayName)
		err = ValidateDisplayName(*params.DisplayName)
		if err != nil {
			err = NewErrorMessage(ErrorInvalidArgument, err.Error())
			return ret, err
		}
	}
	if params.FirstName != nil {
		*params.FirstName = strings.TrimSpace(*params.FirstName)
	}
	if params.LastName != nil {
		*params.LastName = strings.TrimSpace(*params.LastName)
	}

	// start DB transaction
	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("users.UpdateProfile: Starting transaction", rz.Err(err))
		return ret, NewError(ErrorUpdatingProfile)
	}

	if params.ID != nil {
		ret, err = FindUserByID(ctx, tx, *params.ID)
		if err != nil {
			tx.Rollback()
			return ret, err
		}
	} else {
		ret = user
	}

	queryParams := []interface{}{now}
	ret.UpdatedAt = now
	i := 1
	query := fmt.Sprintf("UPDATE users SET updated_at = $%d", i)

	if params.Bio != nil {
		i += 1
		query += fmt.Sprintf(", bio = $%d", i)
		queryParams = append(queryParams, *params.Bio)
		ret.Bio = *params.Bio
	}
	if params.DisplayName != nil {
		i += 1
		query += fmt.Sprintf(", display_name = $%d", i)
		queryParams = append(queryParams, *params.DisplayName)
		ret.DisplayName = *params.DisplayName
	}
	if params.FirstName != nil {
		i += 1
		query += fmt.Sprintf(", first_name = $%d", i)
		queryParams = append(queryParams, *params.FirstName)
		ret.FirstName = *params.FirstName
	}
	if params.LastName != nil {
		i += 1
		query += fmt.Sprintf(", last_name = $%d", i)
		queryParams = append(queryParams, *params.LastName)
		ret.LastName = *params.LastName
	}

	i += 1
	query += fmt.Sprintf(" WHERE id = $%d", i)
	queryParams = append(queryParams, ret.ID)

	fmt.Println("QUERYYY", query)

	_, err = tx.Exec(query, queryParams...)
	if err != nil {
		tx.Rollback()
		logger.Error("users.UpdateProfile: updating profile", rz.Err(err))
		return ret, NewError(ErrorUpdatingProfile)
	}

	// commit db transaction
	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("users.UpdateProfile: Committing transaction", rz.Err(err))
		return ret, NewError(ErrorUpdatingProfile)
	}

	return ret, nil
}

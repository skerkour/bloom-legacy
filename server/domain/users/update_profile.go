package users

import (
	"context"
	"strings"
	"time"

	"gitlab.com/bloom42/bloom/common/validator"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/libs/rz-go"
)

func UpdateProfile(ctx context.Context, user *User, input UpdateProfileInput) (*User, error) {
	var err error
	logger := rz.FromCtx(ctx)
	var ret *User
	now := time.Now().UTC()

	// clean and validate params
	if user == nil {
		logger.Error("", rz.Err(NewError(ErrorUserIsNull)))
		return ret, NewError(ErrorUpdatingProfile)
	}

	if input.ID != nil && (user.ID != *input.ID || !user.IsAdmin) {
		return ret, NewError(ErrorUserNotFound)
	}

	if input.DisplayName == nil && input.FirstName == nil && input.Bio == nil && input.LastName == nil {
		return ret, NewError(ErrorAllFieldsAreEmpty)
	}

	if input.Bio != nil {
		*input.Bio = strings.TrimSpace(*input.Bio)
	}
	if input.DisplayName != nil {
		*input.DisplayName = strings.TrimSpace(*input.DisplayName)
		if err = validator.UserDisplayName(*input.DisplayName); err != nil {
			return ret, NewErrorMessage(ErrorInvalidArgument, err.Error())
		}
	}
	if input.FirstName != nil {
		*input.FirstName = strings.TrimSpace(*input.FirstName)
	}
	if input.LastName != nil {
		*input.LastName = strings.TrimSpace(*input.LastName)
	}

	// start DB transaction
	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("users.UpdateProfile: Starting transaction", rz.Err(err))
		return ret, NewError(ErrorUpdatingProfile)
	}

	if input.ID != nil {
		ret, err = FindUserById(ctx, tx, *input.ID)
		if err != nil {
			tx.Rollback()
			return ret, err
		}
	}

	queryParams := []interface{}{now}
	ret.UpdatedAt = now
	query := "UPDATE users SET (updated_at = ?"

	if input.Bio != nil {
		query += ", bio = ?"
		queryParams = append(queryParams, *input.Bio)
		ret.Bio = *input.Bio
	}
	if input.DisplayName != nil {
		query += ", display_name = ?"
		queryParams = append(queryParams, *input.DisplayName)
		ret.DisplayName = *input.DisplayName
	}
	if input.FirstName != nil {
		query += ", first_name = ?"
		queryParams = append(queryParams, *input.FirstName)
		ret.FirstName = *input.FirstName
	}
	if input.LastName != nil {
		query += ", last_name = ?"
		queryParams = append(queryParams, *input.LastName)
		ret.LastName = *input.LastName
	}

	queryParams = append(queryParams, ret.ID)
	query += ") WHERE id = ?"

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
package billing

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/users"
)

func FindPlans(ctx context.Context, user *users.User) ([]Plan, error) {
	if user == nil || !user.IsAdmin {
		return FindPlansForUser(ctx)
	} else {
		return FindPlansForAdmin(ctx)
	}
}

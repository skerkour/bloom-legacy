package users

import "context"

// CompleteSignInParams are the paramaters for CompleteSignIn
type CompleteSignInParams struct {
}

// CompleteSignIn complete a signin when user use 2fa
func CompleteSignIn(ctx context.Context, params CompleteSignInParams) (user *User, newSession *Session, token string, err error) {
	// globalSessionCache.Set

	return
}

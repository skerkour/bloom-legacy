package query

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/config"
)

// StripePublicKey returns the stripe public key. For billing purpose
func (resolver *Resolver) StripePublicKey(ctx context.Context) (string, error) {
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		return "", gqlerrors.AuthenticationRequired()
	}

	return config.Stripe.PublicKey, nil
}

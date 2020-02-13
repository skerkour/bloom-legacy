package query

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/config"
)

func (resolver *Resolver) StripePublicKey(ctx context.Context) (*string, error) {
	var ret *string
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	ret = &config.Stripe.PublicKey
	return ret, nil
}

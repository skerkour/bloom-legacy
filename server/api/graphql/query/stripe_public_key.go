package query

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/bloom/server/http/httputil"
)

// StripePublicKey returns the stripe public key. For billing purpose
func (resolver *Resolver) StripePublicKey(ctx context.Context) (string, error) {
	httpCtx := httputil.HTTPCtxFromCtx(ctx)

	if httpCtx.AuthenticatedUser == nil {
		return "", api.NewError(errors.PermissionDenied("Authentication required."))
	}

	return resolver.config.Stripe.PublicKey, nil
}

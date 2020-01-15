package accounts

import (
	"context"

	google_protobuf "github.com/golang/protobuf/ptypes/empty"
	"github.com/twitchtv/twirp"
	rpcaccounts "gitlab.com/bloom42/bloom/core/rpc/accounts"
)

// Handler implements the Acounts service
type Handler struct{}

func (s Handler) SignIn(ctx context.Context, params *rpcaccounts.SignInParams) (*rpcaccounts.Session, error) {
	return &rpcaccounts.Session{
		Id:    "MyRandomId",
		Token: "MyRandomToken",
	}, nil
}

func (s Handler) SignOut(ctx context.Context, _ *google_protobuf.Empty) (*google_protobuf.Empty, error) {
	return nil, twirp.NotFoundError("lol not found")
}

package accounts

import (
	"context"

	google_protobuf "github.com/golang/protobuf/ptypes/empty"
	"github.com/twitchtv/twirp"
)

func (s Handler) SignOut(ctx context.Context, _ *google_protobuf.Empty) (*google_protobuf.Empty, error) {
	return nil, twirp.NotFoundError("lol not found")
}

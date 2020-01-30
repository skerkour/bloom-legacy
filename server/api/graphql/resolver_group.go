package graphql

import (
	"context"
	"errors"
)

func (r *groupResolver) Members(ctx context.Context, obj *Group) ([]*GroupMember, error) {
	return nil, errors.New("Forbidden")
}
func (r *groupResolver) Invitations(ctx context.Context, obj *Group) ([]*GroupInvitation, error) {
	panic("not implemented")
}

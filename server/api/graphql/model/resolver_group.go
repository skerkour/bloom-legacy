package model

import (
	"context"
	"errors"
)

type GroupResolver struct{}

func (r *GroupResolver) Members(ctx context.Context, obj *Group) ([]*GroupMember, error) {
	return nil, errors.New("Forbidden")
}
func (r *GroupResolver) Invitations(ctx context.Context, obj *Group) ([]*GroupInvitation, error) {
	panic("not implemented")
}

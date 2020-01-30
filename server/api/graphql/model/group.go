package model

import (
	"context"
	"errors"
	"time"
)

type Group struct {
	ID          *string    `json:"id"`
	CreatedAt   *time.Time `json:"createdAt"`
	Name        string     `json:"name"`
	Description string     `json:"description"`
}

type GroupResolver struct{}

func (r *GroupResolver) Members(ctx context.Context, obj *Group) ([]*GroupMember, error) {
	return nil, errors.New("Forbidden")
}
func (r *GroupResolver) Invitations(ctx context.Context, obj *Group) ([]*GroupInvitation, error) {
	panic("not implemented")
}

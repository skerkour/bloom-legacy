package model

import (
	"context"
	"time"
)

type User struct {
	ID          *string    `json:"id"`
	CreatedAt   *time.Time `json:"createdAt"`
	Username    string     `json:"username"`
	FirstName   *string    `json:"firstName"`
	LastName    *string    `json:"lastName"`
	DisplayName string     `json:"displayName"`
	IsAdmin     bool       `json:"isAdmin"`
}

type UserResolver struct{}

func (resolver *UserResolver) GroupInvitations(ctx context.Context, user *User) ([]*GroupInvitation, error) {
	return nil, nil
}

func (resolver *UserResolver) Groups(ctx context.Context, user *User) ([]*Group, error) {
	return nil, nil
}

func (resolver *UserResolver) Invoices(ctx context.Context, user *User) ([]*Invoice, error) {
	return nil, nil
}

func (resolver *UserResolver) PaymentMethods(ctx context.Context, user *User) ([]*PaymentMethod, error) {
	return nil, nil
}

func (resolver *UserResolver) Sessions(ctx context.Context, user *User) ([]*Session, error) {
	return nil, nil
}

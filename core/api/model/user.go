package model

import (
	"time"
)

type User struct {
	ID               *string                    `json:"id"`
	AvatarURL        *string                    `json:"avatarUrl"`
	CreatedAt        *time.Time                 `json:"createdAt"`
	Username         string                     `json:"username"`
	FirstName        *string                    `json:"firstName"`
	LastName         *string                    `json:"lastName"`
	DisplayName      string                     `json:"displayName"`
	IsAdmin          bool                       `json:"isAdmin"`
	Bio              string                     `json:"bio"`
	Email            *string                    `json:"email"`
	Sessions         *SessionConnection         `json:"sessions"`
	Subscription     *BillingSubscription       `json:"subscription"`
	Invoices         *InvoiceConnection         `json:"invoices"`
	PaymentMethods   *PaymentMethodConnection   `json:"paymentMethods"`
	GroupInvitations *GroupInvitationConnection `json:"groupInvitations"`
	Groups           *GroupConnection           `json:"groups"`
}

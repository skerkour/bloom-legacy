package model

import (
	"time"
)

type Group struct {
	ID             *string                    `json:"id"`
	CreatedAt      *time.Time                 `json:"createdAt"`
	Name           string                     `json:"name"`
	Description    string                     `json:"description"`
	AvatarURL      *string                    `json:"avatarUrl"`
	Members        *GroupMemberConnection     `json:"members"`
	Invitations    *GroupInvitationConnection `json:"invitations"`
	Subscription   *BillingSubscription       `json:"subscription"`
	Invoices       *InvoiceConnection         `json:"invoices"`
	PaymentMethods *PaymentMethodConnection   `json:"paymentMethods"`
}

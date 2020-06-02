package model

import (
	"time"

	"gitlab.com/bloom42/gobox/uuid"
)

type Group struct {
	ID          *uuid.UUID `json:"id"`
	CreatedAt   *time.Time `json:"createdAt"`
	Name        string     `json:"name"`
	Description string     `json:"description"`
	AvatarURL   *string    `json:"avatarUrl"`
	State       *string    `json:"state"`

	EncryptedMasterKey *[]byte `json:"encryptedMasterKey"`
	MasterKeyNonce     *[]byte `json:"masterKeyNonce"`

	Members        *GroupMemberConnection     `json:"members"`
	Invitations    *GroupInvitationConnection `json:"invitations"`
	Subscription   *BillingSubscription       `json:"subscription"`
	Invoices       *InvoiceConnection         `json:"invoices"`
	PaymentMethods *PaymentMethodConnection   `json:"paymentMethods"`
}

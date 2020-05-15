package model

import (
	"time"

	"gitlab.com/bloom42/lily/uuid"
)

type User struct {
	ID          *uuid.UUID `json:"id"`
	AvatarURL   *string    `json:"avatarUrl"`
	CreatedAt   *time.Time `json:"createdAt"`
	Username    string     `json:"username"`
	FirstName   *string    `json:"firstName"`
	LastName    *string    `json:"lastName"`
	DisplayName string     `json:"displayName"`
	IsAdmin     bool       `json:"isAdmin"`
	DisabledAt  *time.Time `json:"disabledAt"`
	Bio         string     `json:"bio"`
	Email       *string    `json:"email"`
	State       *string    `json:"state"`

	PublicKey           []byte `json:"publicKey"`
	EncryptedPrivateKey []byte `json:"encryptedPrivateKey"`
	PrivateKeyNonce     []byte `json:"privateKeyNonce"`
	EncryptedMasterKey  []byte `json:"encryptedMasterKey"`
	MasterKeyNonce      []byte `json:"masterKeyNonce"`

	Sessions         *SessionConnection         `json:"sessions"`
	Subscription     *BillingSubscription       `json:"subscription"`
	Invoices         *InvoiceConnection         `json:"invoices"`
	PaymentMethods   *PaymentMethodConnection   `json:"paymentMethods"`
	GroupInvitations *GroupInvitationConnection `json:"groupInvitations"`
	Groups           *GroupConnection           `json:"groups"`
}

package model

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/domain/sync"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/gobox/uuid"
)

// User is an user
type User struct {
	ID          *uuid.UUID `json:"id"`
	AvatarURL   *string    `json:"avatarUrl"`
	CreatedAt   *time.Time `json:"createdAt"`
	Username    string     `json:"username"`
	FirstName   *string    `json:"firstName"`
	LastName    *string    `json:"lastName"`
	DisplayName string     `json:"displayName"`
	DisabledAt  *time.Time `json:"disabledAt"`
	IsAdmin     bool       `json:"isAdmin"`
	Bio         string     `json:"bio"`
	Email       *string    `json:"email"`
	State       *string    `json:"state"`

	PublicKey           []byte `json:"publicKey"`
	EncryptedPrivateKey []byte `json:"encryptedPrivateKey"`
	PrivateKeyNonce     []byte `json:"privateKeyNonce"`
	EncryptedMasterKey  []byte `json:"encryptedMasterKey"`
	MasterKeyNonce      []byte `json:"masterKeyNonce"`
}

// DomainUserToModelUser transform a `users.User` to `model.User` with the good fields, according to
// actor
func DomainUserToModelUser(actor users.User, user users.User) *User {
	ret := &User{
		AvatarURL:   nil,
		Username:    user.Username,
		DisplayName: user.DisplayName,
		Bio:         user.Bio,
		PublicKey:   user.PublicKey,
	}

	// if same user or admin
	if actor.IsAdmin || actor.ID == user.ID {
		ret.ID = &user.ID
		ret.FirstName = &user.FirstName
		ret.LastName = &user.LastName
		ret.CreatedAt = &user.CreatedAt
		ret.IsAdmin = user.IsAdmin
		ret.Email = &user.Email
		ret.DisabledAt = user.DisabledAt

		// only if same user
		if actor.ID == user.ID {
			var state string

			ret.EncryptedMasterKey = user.EncryptedMasterKey
			ret.MasterKeyNonce = user.MasterKeyNonce
			ret.EncryptedPrivateKey = user.EncryptedPrivateKey
			ret.PrivateKeyNonce = user.PrivateKeyNonce
			state = sync.EncodeState(user.State)
			ret.State = &state
		}
	}

	return ret
}

// UserResolver is the resolver for the User type
type UserResolver struct {
	usersService   users.Service
	groupsService  groups.Service
	billingService billing.Service
}

func NewUserResolver(usersService users.Service, groupsService groups.Service, billingService billing.Service) *UserResolver {
	return &UserResolver{
		usersService:   usersService,
		groupsService:  groupsService,
		billingService: billingService,
	}
}

// GroupInvitations returns the invitations for the user
func (resolver *UserResolver) GroupInvitations(ctx context.Context, user *User) (ret *GroupInvitationConnection, err error) {
	if user.ID == nil {
		err = api.NewError(users.ErrPermissionDenied)
		return
	}

	invitations, err := resolver.groupsService.FindInvitationsForUser(ctx, *user.ID)
	if err != nil {
		err = api.NewError(err)
		return
	}

	ret = &GroupInvitationConnection{
		Nodes:      []*GroupInvitation{},
		TotalCount: int64(len(invitations)),
	}

	for _, invitation := range invitations {
		invit := &GroupInvitation{
			ID: invitation.ID,
			Group: &Group{
				ID:          &invitation.GroupID,
				Name:        invitation.GroupName,
				Description: invitation.GroupDescription,
			},
			Inviter: &User{
				Username:    invitation.InviterUsername,
				DisplayName: invitation.InviterDisplayName,
				PublicKey:   invitation.InviterPublicKey,
			},
			EphemeralPublicKey: &invitation.EphemeralPublicKey,
			Signature:          &invitation.Signature,
			EncryptedMasterKey: &invitation.EncryptedMasterKey,
		}
		ret.Nodes = append(ret.Nodes, invit)
	}
	return ret, nil
}

// Groups returns the groups of the user Groups
func (resolver *UserResolver) Groups(ctx context.Context, user *User) (ret *GroupConnection, err error) {
	if user.ID == nil {
		err = api.NewError(users.ErrPermissionDenied)
		return
	}

	groups, err := resolver.groupsService.FindGroupsForUser(ctx, *user.ID)
	if err != nil {
		err = api.NewError(err)
		return
	}

	ret = &GroupConnection{
		Nodes:      []*Group{},
		TotalCount: int64(len(groups)),
	}

	for i := range groups {
		grp := &Group{
			ID:          &groups[i].ID,
			CreatedAt:   &groups[i].CreatedAt,
			Name:        groups[i].Name,
			Description: groups[i].Description,
		}
		ret.Nodes = append(ret.Nodes, grp)
	}

	return
}

// Invoices return the invoices of the user
func (resolver *UserResolver) Invoices(ctx context.Context, user *User) (ret *InvoiceConnection, err error) {
	if user.ID == nil {
		err = api.NewError(users.ErrPermissionDenied)
		return
	}

	invoices, err := resolver.billingService.FindInvoicesForUser(ctx, *user.ID)
	if err != nil {
		err = api.NewError(err)
		return
	}

	ret = &InvoiceConnection{
		Nodes:      []*Invoice{},
		TotalCount: int64(len(invoices)),
	}

	for _, invoice := range invoices {
		inv := &Invoice{
			ID:              invoice.ID,
			CreatedAt:       invoice.CreatedAt,
			StripePdfURL:    invoice.StripePdfURL,
			PaidAt:          invoice.PaidAt,
			StripeHostedURL: invoice.StripeHostedURL,
			Amount:          invoice.Amount,
		}
		ret.Nodes = append(ret.Nodes, inv)
	}

	return
}

// PaymentMethods returns the payment methods of the user
func (resolver *UserResolver) PaymentMethods(ctx context.Context, user *User) (ret *PaymentMethodConnection, err error) {
	if user.ID == nil {
		err = api.NewError(users.ErrPermissionDenied)
		return
	}

	paymentMethods, err := resolver.billingService.FindPaymentMethodsForUser(ctx, *user.ID)
	if err != nil {
		err = api.NewError(err)
		return
	}

	ret = &PaymentMethodConnection{
		Nodes:      []*PaymentMethod{},
		TotalCount: int64(len(paymentMethods)),
	}

	for _, paymentMethod := range paymentMethods {
		method := &PaymentMethod{
			ID:                  paymentMethod.ID,
			CreatedAt:           paymentMethod.CreatedAt,
			CardLast4:           paymentMethod.CardLast4,
			CardExpirationMonth: int(paymentMethod.CardExpirationMonth),
			CardExpirationYear:  int(paymentMethod.CardExpirationYear),
			IsDefault:           paymentMethod.IsDefault,
		}
		ret.Nodes = append(ret.Nodes, method)
	}

	return ret, nil
}

// Sessions returns the sessions of the user
func (resolver *UserResolver) Sessions(ctx context.Context, user *User) (ret *SessionConnection, err error) {
	if user.ID == nil {
		err = api.NewError(users.ErrPermissionDenied)
		return
	}

	sessions, err := resolver.usersService.FindSessionsForUser(ctx, *user.ID)
	if err != nil {
		err = api.NewError(err)
		return
	}

	ret = &SessionConnection{
		Nodes:      []*Session{},
		TotalCount: int64(len(sessions)),
	}

	for _, session := range sessions {
		sess := &Session{
			ID:        session.ID,
			CreatedAt: session.CreatedAt,
			Token:     nil,
			Device: &SessionDevice{
				Os:   SessionDeviceOs(session.DeviceOS),
				Type: SessionDeviceType(session.DeviceType),
			},
		}
		ret.Nodes = append(ret.Nodes, sess)
	}

	return ret, nil
}

// Subscription returns the subscription of the user
func (resolver *UserResolver) Subscription(ctx context.Context, user *User) (ret *BillingSubscription, err error) {
	var stripePlanID *string
	var stripeCustomerID *string
	var stripeSubscriptionID *string

	if user.ID == nil {
		err = api.NewError(users.ErrPermissionDenied)
		return
	}

	me, err := resolver.usersService.Me(ctx)
	if err != nil {
		err = api.NewError(err)
		return
	}

	customer, plan, err := resolver.billingService.SubscriptionForUser(ctx, *user.ID)
	if err != nil {
		err = api.NewError(err)
		return
	}

	if me.IsAdmin {
		stripePlanID = &plan.StripeID
		stripeCustomerID = customer.StripeCustomerID
		stripeSubscriptionID = customer.StripeSubscriptionID
	}

	ret = &BillingSubscription{
		UpdatedAt:   customer.SubscriptionUpdatedAt,
		UsedStorage: customer.UsedStorage,
		Plan: &BillingPlan{
			ID:          plan.ID,
			Price:       plan.Price,
			Name:        plan.Name,
			Description: plan.Description,
			StripeID:    stripePlanID,
			Product:     BillingProduct(plan.Product),
			Storage:     plan.Storage,
		},
		StripeCustomerID:     stripeCustomerID,
		StripeSubscriptionID: stripeSubscriptionID,
	}
	return
}

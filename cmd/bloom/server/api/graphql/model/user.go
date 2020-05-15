package model

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/db"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/sync"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/users"
	"gitlab.com/bloom42/lily/rz"
	"gitlab.com/bloom42/lily/uuid"
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
func DomainUserToModelUser(actor *users.User, user *users.User) *User {
	ret := &User{
		AvatarURL:   nil,
		Username:    user.Username,
		DisplayName: user.DisplayName,
		Bio:         user.Bio,
		PublicKey:   user.PublicKey,
	}

	// if same user or admin
	if actor != nil && (actor.IsAdmin || actor.ID == user.ID) {
		ret.ID = &user.ID
		ret.FirstName = &user.FirstName
		ret.LastName = &user.LastName
		ret.CreatedAt = &user.CreatedAt
		ret.IsAdmin = user.IsAdmin
		ret.Email = &user.Email
		ret.DisabledAt = user.DisabledAt
	}

	// only if same user
	if actor != nil && actor.ID == user.ID {
		var state string

		ret.EncryptedMasterKey = user.EncryptedMasterKey
		ret.MasterKeyNonce = user.MasterKeyNonce
		ret.EncryptedPrivateKey = user.EncryptedPrivateKey
		ret.PrivateKeyNonce = user.PrivateKeyNonce
		state = sync.EncodeState(user.State)
		ret.State = &state
	}

	return ret
}

// UserResolver is the resolver for the User type
type UserResolver struct{}

type invit struct {
	ID                 uuid.UUID `db:"invitation_id"`
	CreatedAt          time.Time `db:"invitation_created_at"`
	GroupID            uuid.UUID `db:"group_id"`
	GroupCreatedAt     time.Time `db:"group_created_at"`
	GroupName          string    `db:"group_name"`
	GroupDescription   string    `db:"group_description"`
	InviterUsername    string    `db:"inviter_username"`
	InviterDisplayName string    `db:"inviter_display_name"`
}

// GroupInvitations returns the invitations for the user
func (resolver *UserResolver) GroupInvitations(ctx context.Context, user *User) (*GroupInvitationConnection, error) {
	var ret *GroupInvitationConnection
	logger := rz.FromCtx(ctx)
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	if currentUser.ID != uuid.UUID(*user.ID) && !currentUser.IsAdmin {
		return ret, PermissionDeniedToAccessField()
	}

	invitations := []invit{}
	err := db.DB.Select(&invitations, `SELECT invit.id AS invitation_id, invit.created_at AS invitation_created_at,
		groups.id AS group_id, groups.created_at AS group_created_at, groups.name AS group_name, groups.description AS group_description,
			users.username AS inviter_username, users.display_name AS inviter_display_name
			FROM groups_invitations AS invit, groups, users
			WHERE invit.group_id = groups.id AND invit.invitee_id = $1 AND users.id = invit.inviter_id`, user.ID)
	if err != nil {
		logger.Error("groups.ListGroups: fetching invitations", rz.Err(err))
		return ret, gqlerrors.Internal()
	}

	ret = &GroupInvitationConnection{
		Nodes:      []*GroupInvitation{},
		TotalCount: int64(len(invitations)),
	}

	for _, invitation := range invitations {
		invit := &GroupInvitation{
			ID: invitation.ID,
			Group: &Group{
				Name:        invitation.GroupName,
				Description: invitation.GroupDescription,
			},
		}
		ret.Nodes = append(ret.Nodes, invit)
	}
	return ret, nil
}

// Groups returns the groups of the user Groups
func (resolver *UserResolver) Groups(ctx context.Context, user *User) (*GroupConnection, error) {
	var ret *GroupConnection
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	if currentUser.ID != uuid.UUID(*user.ID) && !currentUser.IsAdmin {
		return ret, PermissionDeniedToAccessField()
	}

	logger := rz.FromCtx(ctx)

	groups := []groups.Group{}
	err := db.DB.Select(&groups, `SELECT groups.* FROM groups
		INNER JOIN groups_members ON groups.id = groups_members.group_id
		WHERE groups_members.user_id = $1`, currentUser.ID)
	if err != nil {
		logger.Error("User.groups: fetching groups", rz.Err(err))
		return ret, gqlerrors.Internal()
	}

	ret = &GroupConnection{
		Nodes:      []*Group{},
		TotalCount: int64(len(groups)),
	}

	for _, group := range groups {
		grp := &Group{
			ID:          &group.ID,
			CreatedAt:   &group.CreatedAt,
			Name:        group.Name,
			Description: group.Description,
			//	members: [GroupMember!]
			// invitations: [GroupInvitation!]
		}
		ret.Nodes = append(ret.Nodes, grp)
	}
	return ret, nil
}

// Invoices return the invoices of the user
func (resolver *UserResolver) Invoices(ctx context.Context, user *User) (*InvoiceConnection, error) {
	var ret *InvoiceConnection
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser.ID != uuid.UUID(*user.ID) && !currentUser.IsAdmin {
		return ret, gqlerrors.AdminRoleRequired()
	}

	invoices, err := billing.FindInvoicesByUserId(ctx, nil, uuid.UUID(*user.ID).String())
	if err != nil {
		return ret, gqlerrors.New(err)
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

	return ret, nil
}

// PaymentMethods returns the payment methods of the user
func (resolver *UserResolver) PaymentMethods(ctx context.Context, user *User) (*PaymentMethodConnection, error) {
	var ret *PaymentMethodConnection
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser.ID != uuid.UUID(*user.ID) && !currentUser.IsAdmin {
		return ret, gqlerrors.AdminRoleRequired()
	}

	paymentMethods, err := billing.FindPaymentMethodsByUserId(ctx, nil, uuid.UUID(*user.ID).String())
	if err != nil {
		return ret, gqlerrors.New(err)
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
func (resolver *UserResolver) Sessions(ctx context.Context, user *User) (*SessionConnection, error) {
	var ret *SessionConnection
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser.ID != uuid.UUID(*user.ID) && !currentUser.IsAdmin {
		return ret, gqlerrors.AdminRoleRequired()
	}

	sessions, err := users.FindAllSessionsForUserID(ctx, nil, *user.ID)
	if err != nil {
		return ret, gqlerrors.New(err)
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
func (resolver *UserResolver) Subscription(ctx context.Context, user *User) (*BillingSubscription, error) {
	var ret *BillingSubscription
	currentUser := apiutil.UserFromCtx(ctx)
	var stripePlanID *string
	var stripeCustomerID *string
	var stripeSubscriptionID *string

	if currentUser.ID != uuid.UUID(*user.ID) && !currentUser.IsAdmin {
		return ret, PermissionDeniedToAccessField()
	}

	customer, err := billing.FindCustomerByUserId(ctx, nil, *user.ID, false)
	if err != nil {
		return ret, gqlerrors.New(err)
	}
	plan, err := billing.FindPlanForCustomer(ctx, customer)
	if err != nil {
		return ret, gqlerrors.New(err)
	}

	if currentUser.IsAdmin {
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
			IsPublic:    plan.IsPublic,
			StripeID:    stripePlanID,
			Product:     BillingProduct(plan.Product),
			Storage:     plan.Storage,
		},
		StripeCustomerID:     stripeCustomerID,
		StripeSubscriptionID: stripeSubscriptionID,
	}
	return ret, nil
}

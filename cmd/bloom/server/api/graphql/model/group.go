package model

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/groups"
	"gitlab.com/bloom42/gobox/uuid"
)

// Group is used to share data with other people
type Group struct {
	ID          *uuid.UUID `json:"id"`
	CreatedAt   *time.Time `json:"createdAt"`
	Name        string     `json:"name"`
	Description string     `json:"description"`
	AvatarURL   *string    `json:"avatarUrl"`
	State       *string    `json:"state"`
}

// GroupResolver is the resolver for the Group type
type GroupResolver struct{}

// Members returns the members of the group
func (resolver *GroupResolver) Members(ctx context.Context, group *Group) (*GroupMemberConnection, error) {
	var ret *GroupMemberConnection
	currentUser := apiutil.UserFromCtx(ctx)
	var err error

	if group.ID == nil {
		return ret, PermissionDeniedToAccessField()
	}

	err = groups.CheckUserIsGroupMember(ctx, nil, currentUser.ID, *group.ID)
	if err != nil && !currentUser.IsAdmin {
		return ret, PermissionDeniedToAccessField()
	}

	members, err := groups.FindGroupMembers(ctx, nil, *group.ID)
	if err != nil {
		return ret, gqlerrors.New(err)
	}

	ret = &GroupMemberConnection{
		Edges:      []*GroupMemberEdge{},
		TotalCount: int64(len(members)),
	}

	for _, member := range members {
		usr := &User{
			Username:    member.Username,
			DisplayName: member.DisplayName,
		}
		role := GroupMemberRole(member.Role)
		edge := &GroupMemberEdge{
			Node:     usr,
			Role:     &role,
			JoinedAt: &member.JoinedAt,
		}
		ret.Edges = append(ret.Edges, edge)
	}
	return ret, nil
}

// Invitations returns the pending invitations of the group
func (resolver *GroupResolver) Invitations(ctx context.Context, group *Group) (*GroupInvitationConnection, error) {
	var ret *GroupInvitationConnection
	currentUser := apiutil.UserFromCtx(ctx)
	var err error

	if group.ID == nil {
		return ret, PermissionDeniedToAccessField()
	}

	err = groups.CheckUserIsGroupAdmin(ctx, nil, currentUser.ID, *group.ID)
	if err != nil && !currentUser.IsAdmin {
		return ret, PermissionDeniedToAccessField()
	}

	invitations, err := groups.FindGroupInvitations(ctx, nil, *group.ID)
	if err != nil {
		return ret, gqlerrors.New(err)
	}

	ret = &GroupInvitationConnection{
		Nodes:      []*GroupInvitation{},
		TotalCount: int64(len(invitations)),
	}

	for _, invitation := range invitations {
		invit := &GroupInvitation{
			Inviter: &User{
				Username:    invitation.InviterUsername,
				DisplayName: invitation.InviterUsername,
				AvatarURL:   nil,
			},
			Invitee: &User{
				Username:    invitation.InviteeUsername,
				DisplayName: invitation.InviteeUsername,
				AvatarURL:   nil,
			},
		}
		ret.Nodes = append(ret.Nodes, invit)
	}
	return ret, nil
}

// Subscription returns the subscription of the group
func (resolver *GroupResolver) Subscription(ctx context.Context, group *Group) (*BillingSubscription, error) {
	var ret *BillingSubscription
	currentUser := apiutil.UserFromCtx(ctx)
	var stripePlanID *string
	var stripeCustomerID *string
	var stripeSubscriptionID *string
	var err error

	if group.ID == nil {
		return ret, PermissionDeniedToAccessField()
	}

	err = groups.CheckUserIsGroupAdmin(ctx, nil, currentUser.ID, *group.ID)
	if err != nil && !currentUser.IsAdmin {
		return ret, PermissionDeniedToAccessField()
	}

	customer, err := billing.FindCustomerByGroupID(ctx, nil, *group.ID, false)
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
		UpdatedAt:            customer.SubscriptionUpdatedAt,
		UsedStorage:          customer.UsedStorage,
		StripeCustomerID:     stripeCustomerID,
		StripeSubscriptionID: stripeSubscriptionID,
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
	}
	return ret, nil
}

// Invoices returns the invoices of the group
func (resolver *GroupResolver) Invoices(ctx context.Context, group *Group) (*InvoiceConnection, error) {
	var ret *InvoiceConnection
	currentUser := apiutil.UserFromCtx(ctx)
	var err error

	if group.ID == nil {
		return ret, PermissionDeniedToAccessField()
	}

	err = groups.CheckUserIsGroupAdmin(ctx, nil, currentUser.ID, *group.ID)
	if err != nil && !currentUser.IsAdmin {
		return ret, PermissionDeniedToAccessField()
	}

	invoices, err := billing.FindInvoicesByGroupId(ctx, nil, *group.ID)
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

// PaymentMethods returns the payment methods of the group
func (resolver *GroupResolver) PaymentMethods(ctx context.Context, group *Group) (*PaymentMethodConnection, error) {
	var ret *PaymentMethodConnection
	currentUser := apiutil.UserFromCtx(ctx)
	var err error

	if group.ID == nil {
		return ret, PermissionDeniedToAccessField()
	}

	err = groups.CheckUserIsGroupAdmin(ctx, nil, currentUser.ID, *group.ID)
	if err != nil && !currentUser.IsAdmin {
		return ret, PermissionDeniedToAccessField()
	}

	paymentMethods, err := billing.FindPaymentMethodsByGroupId(ctx, nil, *group.ID)
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

func (resolver *GroupResolver) EncryptedMasterKey(ctx context.Context, group *Group) (*[]byte, error) {
	var ret *[]byte
	currentUser := apiutil.UserFromCtx(ctx)
	var err error

	if group.ID == nil {
		return ret, PermissionDeniedToAccessField()
	}

	err = groups.CheckUserIsGroupMember(ctx, nil, currentUser.ID, *group.ID)
	if err != nil && !currentUser.IsAdmin {
		return ret, PermissionDeniedToAccessField()
	}

	membership, err := groups.FindGroupMasterKey(ctx, nil, *group.ID, currentUser.ID)
	if err != nil {
		return ret, gqlerrors.New(err)
	}

	ret = &membership.EncryptedMasterKey
	return ret, nil
}

func (resolver *GroupResolver) MasterKeyNonce(ctx context.Context, group *Group) (*[]byte, error) {
	var ret *[]byte
	currentUser := apiutil.UserFromCtx(ctx)
	var err error

	if group.ID == nil {
		return ret, PermissionDeniedToAccessField()
	}

	err = groups.CheckUserIsGroupMember(ctx, nil, currentUser.ID, *group.ID)
	if err != nil && !currentUser.IsAdmin {
		return ret, PermissionDeniedToAccessField()
	}

	membership, err := groups.FindGroupMasterKey(ctx, nil, *group.ID, currentUser.ID)
	if err != nil {
		return ret, gqlerrors.New(err)
	}

	ret = &membership.MasterKeyNonce
	return ret, nil
}

package model

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/domain/users"
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
type GroupResolver struct {
	groupsService  groups.Service
	usersService   users.Service
	billingService billing.Service
}

func NewGroupResolver(usersService users.Service, groupsService groups.Service, billingService billing.Service) *GroupResolver {
	return &GroupResolver{
		usersService:   usersService,
		groupsService:  groupsService,
		billingService: billingService,
	}
}

// Members returns the members of the group
func (resolver *GroupResolver) Members(ctx context.Context, group *Group) (ret *GroupMemberConnection, err error) {
	if group.ID == nil {
		err = api.NewError(users.ErrPermissionDenied)
		return
	}

	members, err := resolver.groupsService.GroupMembers(ctx, *group.ID)
	if err == nil {
		err = api.NewError(err)
		return
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
	return
}

// Invitations returns the pending invitations of the group
func (resolver *GroupResolver) Invitations(ctx context.Context, group *Group) (ret *GroupInvitationConnection, err error) {
	if group.ID == nil {
		err = api.NewError(users.ErrPermissionDenied)
		return
	}

	invitations, err := resolver.groupsService.FindInvitationsForGroup(ctx, *group.ID)
	if err == nil {
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
	return
}

// Subscription returns the subscription of the group
func (resolver *GroupResolver) Subscription(ctx context.Context, group *Group) (ret *BillingSubscription, err error) {
	var stripePlanID *string
	var stripeCustomerID *string
	var stripeSubscriptionID *string

	if group.ID == nil {
		err = api.NewError(users.ErrPermissionDenied)
		return
	}

	me, err := resolver.usersService.Me(ctx)
	if err != nil {
		err = api.NewError(err)
		return
	}

	customer, plan, err := resolver.billingService.SubscriptionForGroup(ctx, *group.ID)
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
		UpdatedAt:            customer.SubscriptionUpdatedAt,
		UsedStorage:          customer.UsedStorage,
		StripeCustomerID:     stripeCustomerID,
		StripeSubscriptionID: stripeSubscriptionID,
		Plan: &BillingPlan{
			ID:          plan.ID,
			Price:       plan.Price,
			Name:        plan.Name,
			Description: plan.Description,
			StripeID:    stripePlanID,
			Product:     BillingProduct(plan.Product),
			Storage:     plan.Storage,
		},
	}
	return
}

// Invoices returns the invoices of the group
func (resolver *GroupResolver) Invoices(ctx context.Context, group *Group) (ret *InvoiceConnection, err error) {
	if group.ID == nil {
		err = api.NewError(users.ErrPermissionDenied)
		return
	}

	invoices, err := resolver.billingService.FindInvoicesForGroup(ctx, *group.ID)
	if err == nil {
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

	return ret, nil
}

// PaymentMethods returns the payment methods of the group
func (resolver *GroupResolver) PaymentMethods(ctx context.Context, group *Group) (ret *PaymentMethodConnection, err error) {
	if group.ID == nil {
		err = api.NewError(users.ErrPermissionDenied)
		return
	}

	paymentMethods, err := resolver.billingService.FindPaymentMethodsForGroup(ctx, *group.ID)
	if err == nil {
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

	return
}

func (resolver *GroupResolver) EncryptedMasterKey(ctx context.Context, group *Group) (ret *[]byte, err error) {
	if group.ID == nil {
		err = api.NewError(users.ErrPermissionDenied)
		return
	}

	membership, err := resolver.groupsService.Membership(ctx, *group.ID)
	if err == nil {
		err = api.NewError(err)
		return
	}

	ret = &membership.EncryptedMasterKey
	return
}

func (resolver *GroupResolver) MasterKeyNonce(ctx context.Context, group *Group) (ret *[]byte, err error) {
	if group.ID == nil {
		err = api.NewError(users.ErrPermissionDenied)
		return
	}

	membership, err := resolver.groupsService.Membership(ctx, *group.ID)
	if err == nil {
		err = api.NewError(err)
		return
	}

	ret = &membership.MasterKeyNonce
	return
}

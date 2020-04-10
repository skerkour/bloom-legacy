package model

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/groups"
)

type Group struct {
	ID          *string    `json:"id"`
	CreatedAt   *time.Time `json:"createdAt"`
	Name        string     `json:"name"`
	Description string     `json:"description"`
	AvatarURL   *string    `json:"avatarUrl"`
}

type GroupResolver struct{}

func (r *GroupResolver) Members(ctx context.Context, group *Group) (*GroupMemberConnection, error) {
	var ret *GroupMemberConnection
	currentUser := apiutil.UserFromCtx(ctx)
	var err error

	if group.ID == nil {
		return ret, PermissionDeniedToAccessField()
	}

	err = groups.CheckUserIsGroupMemberNoTx(ctx, currentUser.ID, *group.ID)
	if err != nil && !currentUser.IsAdmin {
		return ret, PermissionDeniedToAccessField()
	}

	members, err := groups.FindGroupMembers(ctx, nil, *group.ID)
	if err != nil {
		return ret, gqlerrors.New(err)
	}

	ret = &GroupMemberConnection{
		Edges:      []*GroupMemberEdge{},
		TotalCount: Int64(len(members)),
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

func (r *GroupResolver) Invitations(ctx context.Context, group *Group) (*GroupInvitationConnection, error) {
	var ret *GroupInvitationConnection
	currentUser := apiutil.UserFromCtx(ctx)
	var err error

	if group.ID == nil {
		return ret, PermissionDeniedToAccessField()
	}

	err = groups.CheckUserIsGroupMemberNoTx(ctx, currentUser.ID, *group.ID)
	if err != nil && !currentUser.IsAdmin {
		return ret, PermissionDeniedToAccessField()
	}

	invitations, err := groups.FindGroupInvitations(ctx, nil, *group.ID)
	if err != nil {
		return ret, gqlerrors.New(err)
	}

	ret = &GroupInvitationConnection{
		Edges:      []*GroupInvitationEdge{},
		TotalCount: Int64(len(invitations)),
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
		edge := &GroupInvitationEdge{
			Node: invit,
		}
		ret.Edges = append(ret.Edges, edge)
	}
	return ret, nil
}

func (resolver *GroupResolver) Subscription(ctx context.Context, group *Group) (*BillingSubscription, error) {
	var ret *BillingSubscription
	currentUser := apiutil.UserFromCtx(ctx)
	var stripePlanId *string
	var stripeCustomerId *string
	var stripeSubscriptionId *string
	var err error

	if group.ID == nil {
		return ret, PermissionDeniedToAccessField()
	}

	err = groups.CheckUserIsGroupAdminNoTx(ctx, currentUser.ID, *group.ID)
	if err != nil && !currentUser.IsAdmin {
		return ret, PermissionDeniedToAccessField()
	}

	customer, err := billing.FindCustomerByGroupIdNoTx(ctx, *group.ID)
	if err != nil {
		return ret, gqlerrors.New(err)
	}
	plan, err := billing.FindPlanForCustomer(ctx, customer)
	if err != nil {
		return ret, gqlerrors.New(err)
	}

	if currentUser.IsAdmin {
		stripePlanId = &plan.StripeID
		stripeCustomerId = customer.StripeCustomerID
		stripeSubscriptionId = customer.StripeSubscriptionID
	}

	ret = &BillingSubscription{
		UpdatedAt:            customer.SubscriptionUpdatedAt,
		UsedStorage:          Int64(customer.UsedStorage),
		StripeCustomerID:     stripeCustomerId,
		StripeSubscriptionID: stripeSubscriptionId,
		Plan: &BillingPlan{
			ID:          plan.ID,
			Price:       Int64(plan.Price),
			Name:        plan.Name,
			Description: plan.Description,
			IsPublic:    plan.IsPublic,
			StripeID:    stripePlanId,
			Product:     BillingProduct(plan.Product),
			Storage:     Int64(plan.Storage),
		},
	}
	return ret, nil
}

func (resolver *GroupResolver) Invoices(ctx context.Context, group *Group) (*InvoiceConnection, error) {
	var ret *InvoiceConnection
	currentUser := apiutil.UserFromCtx(ctx)
	var err error

	if group.ID == nil {
		return ret, PermissionDeniedToAccessField()
	}

	err = groups.CheckUserIsGroupAdminNoTx(ctx, currentUser.ID, *group.ID)
	if err != nil && !currentUser.IsAdmin {
		return ret, PermissionDeniedToAccessField()
	}

	invoices, err := billing.FindInvoicesByGroupId(ctx, nil, *group.ID)
	if err != nil {
		return ret, gqlerrors.New(err)
	}

	ret = &InvoiceConnection{
		Edges:      []*InvoiceEdge{},
		TotalCount: Int64(len(invoices)),
	}

	for _, invoice := range invoices {
		inv := &Invoice{
			ID:              invoice.ID,
			CreatedAt:       invoice.CreatedAt,
			StripePdfURL:    invoice.StripePdfURL,
			Paid:            invoice.Paid,
			StripeHostedURL: invoice.StripeHostedURL,
			Amount:          Int64(invoice.Amount),
		}
		edge := &InvoiceEdge{
			Node: inv,
		}
		ret.Edges = append(ret.Edges, edge)
	}

	return ret, nil
}

func (resolver *GroupResolver) PaymentMethods(ctx context.Context, group *Group) (*PaymentMethodConnection, error) {
	var ret *PaymentMethodConnection
	currentUser := apiutil.UserFromCtx(ctx)
	var err error

	if group.ID == nil {
		return ret, PermissionDeniedToAccessField()
	}

	err = groups.CheckUserIsGroupAdminNoTx(ctx, currentUser.ID, *group.ID)
	if err != nil && !currentUser.IsAdmin {
		return ret, PermissionDeniedToAccessField()
	}

	paymentMethods, err := billing.FindPaymentMethodsByGroupId(ctx, nil, *group.ID)
	if err != nil {
		return ret, gqlerrors.New(err)
	}

	ret = &PaymentMethodConnection{
		Edges:      []*PaymentMethodEdge{},
		TotalCount: Int64(len(paymentMethods)),
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
		edge := &PaymentMethodEdge{
			Node: method,
		}
		ret.Edges = append(ret.Edges, edge)
	}

	return ret, nil
}

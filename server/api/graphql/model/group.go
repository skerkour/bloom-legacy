package model

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/domain/groups"
)

type Group struct {
	ID          *string    `json:"id"`
	CreatedAt   *time.Time `json:"createdAt"`
	Name        string     `json:"name"`
	Description string     `json:"description"`
	AvatarURL   *string    `json:"avatarUrl"`
}

type GroupResolver struct{}

func (r *GroupResolver) Members(ctx context.Context, obj *Group) (*GroupMemberConnection, error) {
	panic("not implemented")
}

func (r *GroupResolver) Invitations(ctx context.Context, obj *Group) (*GroupInvitationConnection, error) {
	panic("not implemented")
	// logger := rz.FromCtx(ctx)
	// apiCtx, ok := ctx.Value(apictx.Key).(*apictx.Context)
	// if !ok {
	// 	return ret, twirp.InternalError("internal error")
	// }
	// if apiCtx.AuthenticatedUser == nil {
	// 	twerr := twirp.NewError(twirp.Unauthenticated, "authentication required")
	// 	return ret, twerr
	// }

	// tx, err := db.DB.Beginx()
	// if err != nil {
	// 	logger.Error("groups.ListGroupInvitations: Starting transaction", rz.Err(err))
	// 	return ret, twirp.InternalError("internal error")
	// }

	// if twerr := groups.CheckUserIsGroupAdmin(ctx, tx, apiCtx.AuthenticatedUser.ID, params.GroupId); twerr != nil {
	// 	tx.Rollback()
	// 	return ret, twerr
	// }

	// invitations := []invit{}
	// err = tx.Select(&invitations, `SELECT invit.id AS invitation_id, invit.created_at AS invitation_created_at,
	// groups.id AS group_id, groups.created_at AS group_created_at, groups.name AS group_name, groups.description AS group_description,
	// 	users.username AS inviter_username, users.display_name AS inviter_display_name
	// 	FROM groups_invitations AS invit, groups, users
	// 	WHERE invit.group_id = $1 AND users.id = invit.inviter_id`, params.GroupId)
	// if err != nil {
	// 	tx.Rollback()
	// 	logger.Error("groups.ListGroupInvitations: fetching invitations", rz.Err(err))
	// 	return ret, twirp.InternalError(groups.ErrorListingInvitationsMsg)
	// }

	// err = tx.Commit()
	// if err != nil {
	// 	tx.Rollback()
	// 	logger.Error("groups.ListGroupInvitations: Committing transaction", rz.Err(err))
	// 	return ret, twirp.InternalError(groups.ErrorListingInvitationsMsg)
	// }

	// for _, invitation := range invitations {
	// 	ret.Invitations = append(ret.Invitations, invitToRpcInvitation(invitation))
	// }
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

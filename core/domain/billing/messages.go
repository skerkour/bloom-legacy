package billing

import (
	"gitlab.com/bloom42/bloom/core/api/model"
)

type MyBillingProfile struct {
	Me           *model.User          `json:"me"`
	BillingPlans *[]model.BillingPlan `json:"billingPlans"`
}

type UserBillingProfile struct {
	User         *model.User          `json:"user"`
	BillingPlans *[]model.BillingPlan `json:"billingPlans"`
}

type GroupBillingProfile struct {
	Group        *model.Group         `json:"group"`
	BillingPlans *[]model.BillingPlan `json:"billingPlans"`
}

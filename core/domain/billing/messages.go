package billing

import (
	"gitlab.com/bloom42/bloom/core/api/model"
)

type MyBillingProfile struct {
	Me           *model.User                  `json:"me"`
	BillingPlans *model.BillingPlanConnection `json:"billingPlans"`
}

type UserBillingProfile struct {
	User         *model.User                  `json:"user"`
	BillingPlans *model.BillingPlanConnection `json:"billingPlans"`
}

type GroupBillingProfile struct {
	Group        *model.Group                 `json:"group"`
	BillingPlans *model.BillingPlanConnection `json:"billingPlans"`
}

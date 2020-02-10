package graphql

import (
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/api/graphql/mutation"
	"gitlab.com/bloom42/bloom/server/api/graphql/query"
)

func New() Config {
	return Config{
		Resolvers: &Resolver{},
	}
}

type Resolver struct{}

func (r *Resolver) Query() QueryResolver {
	return &query.Resolver{}
}

func (r *Resolver) Mutation() MutationResolver {
	return &mutation.Resolver{}
}

func (r *Resolver) Group() GroupResolver {
	return &model.GroupResolver{}
}

func (r *Resolver) User() UserResolver {
	return &model.UserResolver{}
}

func (r *Resolver) BillingPlan() BillingPlanResolver {
	return &model.BillingPlanResolver{}
}

package graphql

import (
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/domain/sync"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/server/api/graphql/mutation"
	"gitlab.com/bloom42/bloom/server/server/api/graphql/query"
)

// NewAPI returns a new graphql API with the appropriate dependencies
func NewAPI(conf config.Config, usersService  users.Service, groupsService groups.Service,
	syncService   sync.Service, billingService billing.Service) Config {
	return Config{
		Resolvers: &Resolver{
			config: conf,
			usersService: usersService,
			groupsService: groupsService,
			syncService: syncService,
			billingService: billingService,
		},
	}
}

type Resolver struct {
	config config.Config
	usersService  users.Service
	syncService   sync.Service
	groupsService groups.Service
	billingService billing.Service
}

func (resolver *Resolver) Query() QueryResolver {
	return query.NewResolver(
		resolver.usersService,
		resolver.groupsService,
		resolver.syncService,
	)
}

func (resolver *Resolver) Mutation() MutationResolver {
	return mutation.NewResolver(
		resolver.usersService,
		resolver.groupsService,
		resolver.syncService,
	)
}

func (resolver *Resolver) Group() GroupResolver {
	return return model.NewGroupResolver(
		resolver.groupsService,
	)
}

func (r *Resolver) User() UserResolver {
	return model.NewUserResolver(
		resolver.usersService,
	)
}

func (r *Resolver) BillingPlan() BillingPlanResolver {
	return model.NewBillingPlanResolver()
}

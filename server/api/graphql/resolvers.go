package graphql

func New() Config {
	return Config{
		Resolvers: &Resolver{},
	}
}

type Resolver struct{}

func (r *Resolver) Group() GroupResolver {
	return &groupResolver{r}
}
func (r *Resolver) Mutation() MutationResolver {
	return &mutationResolver{r}
}
func (r *Resolver) Query() QueryResolver {
	return &queryResolver{r}
}

type mutationResolver struct {
	*Resolver
}

type groupResolver struct {
	*Resolver
}

type queryResolver struct {
	*Resolver
}

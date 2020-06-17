package repository

// GroupsRepository is an implementation of `groups.Repository`
type GroupsRepository struct {
}

// NewGroupsRepository returns a new instance of GroupsRepository
func NewGroupsRepository() *GroupsRepository {
	return &GroupsRepository{}
}

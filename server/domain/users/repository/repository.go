package repository

import (
	"gitlab.com/bloom42/megabox/server/driver"
)

// UsersRepository is an implementation of `users.Repository`
type UsersRepository struct {
	cache driver.Cache
}

// NewUsersRepository returns a new instance of UsersRepository
func NewUsersRepository(cache driver.Cache) *UsersRepository {
	return &UsersRepository{
		cache: cache,
	}
}

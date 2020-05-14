package objects

import (
	"context"
)

// Init inits the objects (sync) service
func Init() error {
	Sync = make(chan bool)

	ctx := context.Background()
	go backgroundSync(ctx)

	return nil
}

package objects

import (
	"context"
)

// Init inits the objects (sync) service
func Init() error {
	SyncChan = make(chan bool)

	go backgroundSync(context.Background())

	return nil
}

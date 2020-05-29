package objects

import (
	"context"
)

// Init inits the objects (sync) service
func Init(enableBackgroundSync bool) error {
	SyncChan = make(chan bool)

	if enableBackgroundSync {
		go backgroundSync(context.Background())
	}
	return nil
}

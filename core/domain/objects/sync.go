package objects

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/core/domain/kernel"
	"gitlab.com/bloom42/gobox/rz"
	"gitlab.com/bloom42/gobox/rz/log"
)

// Sync sync the local data with the pod (pull + conflict resolution + push)
// it should only be manually called after signing in to sync data
func Sync() (err error) {
	syncyingMutext.Lock()
	defer syncyingMutext.Unlock()

	// sync only if user is authenticated
	if kernel.Me != nil {
		err = pull()
		if err != nil {
			log.Debug("Error pulling data", rz.Err(err))
			return
		}
		err = push()
		if err != nil {
			log.Debug("Error pushing data", rz.Err(err))
		}
	}
	return
}

// backgroundSync is a worker that do background sync
func backgroundSync(ctx context.Context) {
	ticker := time.NewTicker(10 * time.Second)

	for {
		select {
		case <-ctx.Done():
			// cleanup
			ticker.Stop()
			break
		case <-ticker.C:
			Sync()
		case <-SyncChan:
			Sync()
		}
	}
}

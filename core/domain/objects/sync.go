package objects

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/core/domain/kernel"
)

// backgroundSync is a worker that do background sync
func backgroundSync(ctx context.Context) {
	ticker := time.NewTicker(4 * time.Second)

	for {
		select {
		case <-ctx.Done():
			// cleanup
			ticker.Stop()
			break
		case <-ticker.C:
			backSync()
		case <-Sync:
			backSync()
		}
	}
}

func backSync() {
	syncyingMutext.Lock()
	defer syncyingMutext.Unlock()

	// sync only if user is authenticated
	if kernel.Me != nil {
		err := pull()
		if err != nil {
			return
		}
		push()
	}
}

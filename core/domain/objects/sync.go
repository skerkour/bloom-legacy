package objects

import (
	"context"
	"time"
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
	err := pull()
	if err != nil {
		return
	}
	push()
}

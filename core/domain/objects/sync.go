package objects

import (
	"context"
	"time"
)

// backgroundSync is a worker that do background sync
func backgroundSync(ctx context.Context) {
	ticker := time.NewTicker(3 * time.Second)

	for {
		select {
		case <-ctx.Done():
			// cleanup
			break
		case <-ticker.C:
			err := pull()
			if err != nil {
				continue
			}
			push()
		case <-Sync:
			err := pull()
			if err != nil {
				continue
			}
			push()
		}
	}
}

package objects

import (
	"context"
)

// Init inits the objects (sync) service
func Init() error {
	Sync = make(chan bool)

	currentStates = states{states: map[string]string{}}
	// todo: fetch current states in DB

	ctx := context.Background()
	go backgroundSync(ctx)

	return nil
}

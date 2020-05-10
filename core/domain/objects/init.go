package objects

import (
	"context"
	"sync"
)

// Sync force a sync
var Sync chan bool

var currentStates states

type states struct {
	mutex  sync.RWMutex
	states map[string]string
}

// Init inits the objects (sync) service
func Init() error {
	Sync = make(chan bool)

	currentStates = states{states: map[string]string{}}
	// todo: fetch current states in DB

	ctx := context.Background()
	go backgroundSync(ctx)

	return nil
}

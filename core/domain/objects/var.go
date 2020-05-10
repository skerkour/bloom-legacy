package objects

import (
	"sync"
)

// Sync force a sync
var Sync chan bool

var currentStates states

type states struct {
	mutex  sync.RWMutex
	states map[string]string
}

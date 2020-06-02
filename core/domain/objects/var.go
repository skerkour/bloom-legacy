package objects

import (
	"sync"
)

// SyncChan force a sync
var SyncChan chan bool

var syncyingMutext sync.Mutex

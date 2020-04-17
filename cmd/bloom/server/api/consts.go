package api

import (
	"time"
)

const (
	HeaderKeyBloomRequestID = "X-Bloom-Request-ID"
	SERVER_READ_TIMEOUT     = 5 * time.Second
	SERVER_WRITE_TIMEOUT    = 10 * time.Second
	SERVER_IDLE_TIMEOUT     = 30 * time.Second
)

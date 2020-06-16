package api

import (
	"time"
)

const (
	// HEADER_BLOOM_REQUEST_ID is the HTTP header key for the request id
	HEADER_BLOOM_REQUEST_ID = "X-Bloom-Request-ID"
	SERVER_READ_TIMEOUT     = 5 * time.Second
	SERVER_WRITE_TIMEOUT    = 10 * time.Second
	SERVER_IDLE_TIMEOUT     = 30 * time.Second
)

package http

import (
	"time"
)

const (
	// HeaderBloomRequestID is the HTTP header where request id is placed
	HeaderBloomRequestID = "X-Bloom-Request-ID"

	// ReadTimeout is the ReadTimeout of the `http.Server`
	ReadTimeout = 5 * time.Second
	// WriteTimeout is the WriteTimeout of the `http.Server`
	WriteTimeout = 10 * time.Second
	// IdleTimeout is the IdleTimeout of the `http.Server`
	IdleTimeout = 30 * time.Second

	// CORSMaxAge is the MaxAge CORS value in secods
	CORSMaxAge = 3600
)

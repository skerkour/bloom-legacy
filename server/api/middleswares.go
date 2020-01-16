package api

import (
	"context"
	"github.com/google/uuid"
	"gitlab.com/bloom42/libs/rz-go"
	"gitlab.com/bloom42/libs/rz-go/rzhttp"
	"net"
	"net/http"
)

func SetSecurityHeadersMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("X-Content-Type-Options", "nosniff")
		w.Header().Set("X-Frame-Options", "Deny")
		w.Header().Set("Strict-Transport-Security", "max-age=63072000; includeSubDomains; preload")
		next.ServeHTTP(w, r)
	})
}

func SetRequestIDMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {

		uuidv4, _ := uuid.NewRandom()
		requestID := uuidv4.String()

		ctx := r.Context()
		ctx = context.WithValue(ctx, rzhttp.RequestIDCtxKey, requestID)
		w.Header().Set(HeaderKeyBloomRequestID, requestID)

		next.ServeHTTP(w, r.WithContext(ctx))
	})
}

func SetContextMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		var err error

		ctx := r.Context()
		apiCtx := Context{}
		apiCtx.RequestID = ctx.Value(rzhttp.RequestIDCtxKey).(string)

		remote := r.RemoteAddr
		host, _, err := net.SplitHostPort(remote)
		if err == nil {
			remote = host
		}
		// check that remote address is valid
		// if err = validator.IP(remote); err != nil {
		// 	InternalError(w, r)
		// 	return
		// }
		apiCtx.IP = remote
		apiCtx.UserAgent = r.UserAgent()

		ctx = context.WithValue(ctx, CtxKey, &apiCtx)

		next.ServeHTTP(w, r.WithContext(ctx))
	})
}

func SetLoggerMiddleware(logger rz.Logger) func(next http.Handler) http.Handler {
	return func(next http.Handler) http.Handler {
		return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			if rid, ok := r.Context().Value(rzhttp.RequestIDCtxKey).(string); ok {
				logger = logger.With(rz.Fields(rz.String("request_id", rid)))
				ctx := logger.ToCtx(r.Context())
				r = r.WithContext(ctx)
			}

			next.ServeHTTP(w, r)
		})
	}
}

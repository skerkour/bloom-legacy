package http

import (
	"context"
	"net"
	"net/http"
	"strings"

	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/http/httpctx"
	"gitlab.com/bloom42/bloom/server/http/httputil"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/log/loghttp"
	"gitlab.com/bloom42/gobox/uuid"
)

// MiddlewareSetSecurityHeaders sets the `X-Content-Type-Options`, `X-Frame-Options`,
// `Strict-Transport-Security` security headers
func (server *Server) MiddlewareSetSecurityHeaders(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("X-Content-Type-Options", "nosniff")
		w.Header().Set("X-Frame-Options", "Deny")
		w.Header().Set("Strict-Transport-Security", "max-age=63072000; includeSubDomains; preload")
		next.ServeHTTP(w, r)
	})
}

// MiddlewareSetRequestID injects a random UUIDv4 in each request and set it as header with the
// `HeaderBloomRequestID` key
func (server *Server) MiddlewareSetRequestID(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {

		requestID := uuid.New()

		ctx := r.Context()
		ctx = context.WithValue(ctx, loghttp.RequestIDCtxKey, requestID)
		w.Header().Set(HeaderBloomRequestID, requestID.String())

		next.ServeHTTP(w, r.WithContext(ctx))
	})
}

// MiddlewareSetHTTPContext injects `httpctx.Context` in requests' context
func (server *Server) MiddlewareSetHTTPContext(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		var err error

		ctx := r.Context()
		httpCtx := httpctx.Context{}
		httpCtx.RequestID = ctx.Value(loghttp.RequestIDCtxKey).(uuid.UUID)

		host, _, err := net.SplitHostPort(r.RemoteAddr)
		if err == nil {
			remoteIP := net.ParseIP(host)
			if remoteIP != nil {
				httpCtx.IP = host
			}
		}
		httpCtx.UserAgent = r.UserAgent()

		ctx = context.WithValue(ctx, httpctx.Key, &httpCtx)

		next.ServeHTTP(w, r.WithContext(ctx))
	})
}

// MiddlewareSetLogger injects `server.logger` in the context of requests
func (server *Server) MiddlewareSetLogger(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if rid, ok := r.Context().Value(loghttp.RequestIDCtxKey).(uuid.UUID); ok {
			logger := server.logger.Clone(log.SetFields(log.String("request_id", rid.String())))
			ctx := logger.ToCtx(r.Context())
			r = r.WithContext(ctx)
		}

		next.ServeHTTP(w, r)
	})
}

// MiddlewareAuth is a middleware which checks the `Authorizartion` header. If data is provided the
// middleware verifies that the data is correct and then fill the context of the current request
func (server *Server) MiddlewareAuth(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {

		reqCtx := r.Context()

		httpCtx := httputil.HTTPCtxFromCtx(r.Context())
		authHeader := r.Header.Get("authorization")

		if authHeader != "" {
			_, token, err := server.decodeAuthorizationHeader(authHeader)
			if err != nil {
				server.SendError(w, r, http.StatusUnauthorized, err)
				return
			}
			currentUser, currentSession, err := server.usersService.VerifySessionToken(reqCtx, token)
			if err != nil {
				server.SendError(w, r, http.StatusUnauthorized, err)
				return
			}
			httpCtx.AuthenticatedUser = &currentUser
			httpCtx.Session = &currentSession
		}

		next.ServeHTTP(w, r.WithContext(reqCtx))
	})
}

func (server *Server) decodeAuthorizationHeader(header string) (tokenType, token string, err error) {
	header = strings.TrimSpace(header)
	parts := strings.Split(header, " ")
	if len(parts) != 2 {
		err = users.ErrInvalidSession
		return
	}
	tokenType = strings.ToLower(parts[0])
	token = parts[1]

	if tokenType != "basic" {
		err = users.ErrInvalidSession
		return
	}

	return
}

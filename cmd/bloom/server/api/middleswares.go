package api

import (
	"context"
	"encoding/json"
	"net"
	"net/http"
	"strings"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/apictx"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/config"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/users"
	"gitlab.com/bloom42/lily/crypto"
	"gitlab.com/bloom42/lily/rz"
	"gitlab.com/bloom42/lily/rz/rzhttp"
	"gitlab.com/bloom42/lily/uuid"
)

// SetSecurityHeadersMiddleware sets some security headers
func SetSecurityHeadersMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("X-Content-Type-Options", "nosniff")
		w.Header().Set("X-Frame-Options", "Deny")
		w.Header().Set("Strict-Transport-Security", "max-age=63072000; includeSubDomains; preload")
		next.ServeHTTP(w, r)
	})
}

// SetRequestIDMiddleware injects a random UUIDv4 in each request and set it as header with the
// `HEADER_BLOOM_REQUEST_ID` key
func SetRequestIDMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {

		requestID := uuid.New()

		ctx := r.Context()
		ctx = context.WithValue(ctx, rzhttp.RequestIDCtxKey, requestID)
		w.Header().Set(HEADER_BLOOM_REQUEST_ID, requestID.String())

		next.ServeHTTP(w, r.WithContext(ctx))
	})
}

// SetContextMiddleware injects `apictx.Context` in requests' context
func SetContextMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		var err error

		ctx := r.Context()
		apiCtx := apictx.Context{}
		apiCtx.RequestID = ctx.Value(rzhttp.RequestIDCtxKey).(uuid.UUID)

		remote := r.RemoteAddr
		host, _, err := net.SplitHostPort(remote)
		if err == nil {
			remote = host
		}
		// check that remote address is valid
		if err = ValidateIP(remote); err != nil {
			erro := Error{
				Extensions: map[string]string{
					"code": "invalid_argument",
				},
				Message: "remote IP address is not valid",
			}
			ResError(w, r, http.StatusInternalServerError, erro)
			return
		}
		apiCtx.IP = remote
		apiCtx.UserAgent = r.UserAgent()

		ctx = context.WithValue(ctx, apictx.Key, &apiCtx)

		next.ServeHTTP(w, r.WithContext(ctx))
	})
}

// SetLoggerMiddleware injects `logger` in the context of requests
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

type graphqlRes struct {
	Data   interface{}    `json:"data"`
	Errors []graphqlError `json:"errors"`
}

type graphqlError struct {
	Message    string            `json:"message"`
	Path       []string          `json:"path"`
	Extensions map[string]string `json:"extensions"`
}

func invalidSession(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "application/json")
	code := "permission_denied"
	message := "Session is not valid"

	b, err := json.Marshal(graphqlRes{
		Data: nil,
		Errors: []graphqlError{
			{Message: message, Path: []string{}, Extensions: map[string]string{"code": code}},
		},
	})
	if err != nil {
		http.Error(w, err.Error(), 500)
		return
	}
	w.WriteHeader(200)
	w.Write(b)
}

// AuthMiddleware is a middleware which checks the `Authorizartion`. If data is provided the
// middleware verifies that the data is correct and then fill the context of the current request
func AuthMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		currentUser := &users.User{}

		reqCtx := r.Context()

		apiCtx := apiutil.ApiCtxFromCtx(r.Context())
		authHeader := r.Header.Get("authorization")

		if authHeader != "" {

			parts := strings.FieldsFunc(authHeader, isSpace)
			if len(parts) != 2 || (parts[0] != "Basic" && parts[0] != "Secret") {
				invalidSession(w, r)
				return
			}

			if parts[0] == "Basic" {

				sessionID, sessionSecret, err := users.ParseSessionToken(parts[1])
				if err != nil {
					invalidSession(w, r)
					return
				}
				currentSession, err := users.VerifySession(sessionID, sessionSecret)
				// remove sessionSecret from memory
				crypto.Zeroize(sessionSecret)
				if err != nil {
					invalidSession(w, r)
					return
				}

				currentUser, err = users.FindUserByID(reqCtx, nil, currentSession.UserID)
				if err != nil {
					invalidSession(w, r)
					return
				}
				apiCtx.AuthenticatedUser = currentUser
				apiCtx.Session = currentSession

				// update session's fields if necessary
				// go session.Access(reqCtx, *authenticatedAccount, *currentSession, ctx.IP, ctx.UserAgent, ctx.RequestID)

			} else { // Secret
				secret := parts[1]
				if secret == config.Bitflow.Secret {
					service := "bitflow"
					apiCtx.AuthenticatedService = &service
				} else {
					invalidSession(w, r)
					return
				}
			}
		}

		next.ServeHTTP(w, r.WithContext(reqCtx))
	})
}

func isSpace(c rune) bool {
	return c == ' '
}

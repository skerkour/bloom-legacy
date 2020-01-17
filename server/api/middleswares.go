package api

import (
	"context"
	"net"
	"net/http"

	"github.com/google/uuid"
	"gitlab.com/bloom42/bloom/server/api/apictx"
	"gitlab.com/bloom42/libs/rz-go"
	"gitlab.com/bloom42/libs/rz-go/rzhttp"
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
		apiCtx := apictx.Context{}
		apiCtx.RequestID = ctx.Value(rzhttp.RequestIDCtxKey).(string)

		remote := r.RemoteAddr
		host, _, err := net.SplitHostPort(remote)
		if err == nil {
			remote = host
		}
		// check that remote address is valid
		if err = ValidateIP(remote); err != nil {
			erro := Error{
				Code:    "invalid_argument",
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

/*
func AuthMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		authenticatedAccount := &domain.Account{}
		currentSession := &domain.Session{}

		reqCtx := r.Context()

		ctx := reqCtx.Value("api_context").(*apictx.Context)
		authHeader := r.Header.Get("authorization")

		if authHeader != "" {
			parts := strings.FieldsFunc(authHeader, isSpace)
			if len(parts) != 2 || (parts[0] != "Basic" && parts[0] != "Secret") {
				InvalidAuthorizationHeader(w, r)
				return
			}

			if parts[0] == "Basic" {

			sessionID, sessionToken, err := util.DecodeSession(parts[1])
			if err != nil {
				InvalidToken(w, r)
				return
			}

			currentSession.ID = sessionID
			goes.DB.First(currentSession).Related(authenticatedAccount)
			if authenticatedAccount.ID != "" {
				if util.VerifyHashedString(currentSession.Token, sessionToken) != true {
					// given sessionToken does not match with the actual hashed sesisonToken
					InvalidSession(w, r)
					return
				}
				ctx.AuthenticatedAccount = authenticatedAccount
				ctx.Session = currentSession

				// update session's fields if necessary
				go session.Access(reqCtx, *authenticatedAccount, *currentSession, ctx.IP, ctx.UserAgent, ctx.RequestID)
			} else {
				// session not found or not active
				InvalidSession(w, r)
				return
			}
			} else { // Secret
				secret := parts[1]
				if secret == config.BitflowSecret {
					service := "bitflow"
					ctx.AuthenticatedService = &service
				} else if secret == config.PhaserSecret {
					service := "phaser"
					ctx.AuthenticatedService = &service
				} else {
					InvalidSecret(w, r)
					return
				}
			}
		}

		next.ServeHTTP(w, r.WithContext(reqCtx))
	})
}
*/

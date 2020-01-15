package main

import (
	"context"
	"fmt"
	"net/http"
	"os"

	"github.com/go-chi/chi"
	google_protobuf "github.com/golang/protobuf/ptypes/empty"
	"github.com/google/uuid"
	"github.com/twitchtv/twirp"
	rpcaccounts "gitlab.com/bloom42/bloom/core/rpc/accounts"
	"gitlab.com/bloom42/libs/rz-go"
	"gitlab.com/bloom42/libs/rz-go/log"
	"gitlab.com/bloom42/libs/rz-go/rzhttp"
)

// AccountsServer implements the Haberdasher service
type AccountsServer struct{}

func (s AccountsServer) SignIn(ctx context.Context, params *rpcaccounts.SignInParams) (*rpcaccounts.Session, error) {
	return nil, twirp.NotFoundError("lol not found")
}

func (s AccountsServer) SignOut(ctx context.Context, _ *google_protobuf.Empty) (*google_protobuf.Empty, error) {
	return nil, twirp.NotFoundError("lol not found")
}

func main() {
	env := os.Getenv("GO_ENV")
	port := os.Getenv("PORT")
	if port == "" {
		port = "8000"
	}
	twirpHandler := rpcaccounts.NewAccountsServer(AccountsServer{}, nil)

	log.SetLogger(log.With(
		rz.Fields(
			rz.String("service", "api"), rz.String("host", "abcd.local"), rz.String("env", env),
		),
	))

	router := chi.NewRouter()

	// replace size field name by latency and disable userAgent logging
	loggingMiddleware := rzhttp.Handler(log.Logger(), rzhttp.Duration("latency"), rzhttp.UserAgent(""))

	// here the order matters, otherwise loggingMiddleware won't see the request ID
	router.Use(requestIDMiddleware)
	router.Use(loggingMiddleware)
	router.Use(injectLoggerMiddleware(log.Logger()))

	router.Get("/", helloWorld)
	router.Mount(twirpHandler.PathPrefix(), twirpHandler)

	err := http.ListenAndServe(":"+port, router)
	if err != nil {
		log.Fatal("listening", rz.Err(err))
	}
}

func requestIDMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		uuidv4, _ := uuid.NewRandom()
		requestID := uuidv4.String()
		w.Header().Set("X-Bloom-Request-ID", requestID)

		ctx := context.WithValue(r.Context(), rzhttp.RequestIDCtxKey, requestID)
		next.ServeHTTP(w, r.WithContext(ctx))
	})
}

func injectLoggerMiddleware(logger rz.Logger) func(next http.Handler) http.Handler {
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

func helloWorld(w http.ResponseWriter, r *http.Request) {
	// logger := rz.FromCtx(r.Context())
	// logger.Info("hello from GET /")
	fmt.Fprintf(w, "Hello, you've requested: %s\n", r.URL.Path)
}

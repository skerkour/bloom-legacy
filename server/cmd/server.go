package cmd

import (
	"context"
	"encoding/json"
	"net/http"
	"os"

	"github.com/go-chi/chi"
	"github.com/google/uuid"
	"github.com/spf13/cobra"
	rpcaccounts "gitlab.com/bloom42/bloom/core/rpc/accounts"
	"gitlab.com/bloom42/bloom/server/bloom/accounts"
	"gitlab.com/bloom42/libs/rz-go"
	"gitlab.com/bloom42/libs/rz-go/log"
	"gitlab.com/bloom42/libs/rz-go/rzhttp"
)

func init() {
	rootCmd.AddCommand(serverCmd)
}

var serverCmd = &cobra.Command{
	Use:     "server",
	Aliases: []string{"server", "s"},
	Short:   "Run the server",
	Long:    "Run the server",
	Run: func(cmd *cobra.Command, args []string) {
		env := os.Getenv("GO_ENV")
		port := os.Getenv("PORT")
		if port == "" {
			port = "8000"
		}

		log.SetLogger(log.With(
			rz.Fields(
				rz.String("service", "api"), rz.String("host", "abcd.local"), rz.String("env", env),
			),
		))

		router := chi.NewRouter()

		// replace size field name by latency and disable userAgent logging
		loggingMiddleware := rzhttp.Handler(log.Logger(), rzhttp.Duration("latency"), rzhttp.UserAgent(""))

		accountsHandler := rpcaccounts.NewAccountsServer(accounts.Handler{}, nil)

		// here the order matters, otherwise loggingMiddleware won't see the request ID
		router.Use(requestIDMiddleware)
		router.Use(loggingMiddleware)
		router.Use(injectLoggerMiddleware(log.Logger()))

		router.Get("/", helloWorld)
		router.Mount(accountsHandler.PathPrefix(), accountsHandler)
		router.NotFound(http.HandlerFunc(notFoundHandler))

		err := http.ListenAndServe(":"+port, router)
		if err != nil {
			log.Fatal("listening", rz.Err(err))
		}
	},
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

type HelloWorld struct {
	Hello string `json:"hello"`
}

func helloWorld(w http.ResponseWriter, r *http.Request) {
	data, err := json.Marshal(HelloWorld{Hello: "world"})
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("content-type", "application/json")
	w.Write(data)
}

type APIError struct {
	Code    string            `json:"code"`
	Message string            `json:"msg"`
	Meta    map[string]string `json:"meta"`
}

func notFoundHandler(w http.ResponseWriter, r *http.Request) {
	data, err := json.Marshal(APIError{
		Code:    "not_found",
		Message: "route not found",
		Meta: map[string]string{
			"path": r.URL.Path,
		},
	})
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	w.Header().Set("content-type", "application/json")
	w.WriteHeader(http.StatusNotFound)
	w.Write(data)
}

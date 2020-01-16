package api

import (
	"fmt"
	"net/http"

	"github.com/go-chi/chi"
	"github.com/go-chi/chi/middleware"
	rpcaccounts "gitlab.com/bloom42/bloom/core/rpc/accounts"
	"gitlab.com/bloom42/bloom/server/bloom/accounts"
	"gitlab.com/bloom42/bloom/server/config"
	"gitlab.com/bloom42/libs/rz-go"
	"gitlab.com/bloom42/libs/rz-go/log"
	"gitlab.com/bloom42/libs/rz-go/rzhttp"
)

func Run() error {
	router := chi.NewRouter()

	// replace size field name by latency and disable userAgent logging
	loggingMiddleware := rzhttp.Handler(log.Logger(), rzhttp.Duration("latency"))

	accountsHandler := rpcaccounts.NewAccountsServer(accounts.Handler{}, nil)

	/*

		router.Use(SetRequestID)
		router.Use(rzhttp.Handler(log.Logger()))
		router.Use(injectLoggerMiddleware(log.Logger()))
		router.Use(middleware.Recoverer)
		router.Use(middleware.Timeout(30 * time.Second))
	*/

	// here the order matters, otherwise loggingMiddleware won't see the request ID
	router.Use(middleware.RealIP)
	router.Use(SetRequestIDMiddleware)
	router.Use(loggingMiddleware)
	router.Use(SetLoggerMiddleware(log.Logger()))
	router.Use(middleware.Recoverer)
	// router.Use(middleware.Timeout(60 * time.Second))

	router.Use(SecurityHeadersMiddleware)

	router.Get("/", HelloWorlHandler)
	router.Mount(accountsHandler.PathPrefix(), accountsHandler)
	router.NotFound(http.HandlerFunc(NotFoundHandler))

	log.Info("starting server", rz.Uint16("port", config.Config.Port))
	return http.ListenAndServe(fmt.Sprintf(":%d", config.Config.Port), router)
}

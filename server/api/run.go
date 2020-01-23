package api

import (
	"fmt"
	"net/http"

	"github.com/go-chi/chi"
	"github.com/go-chi/chi/middleware"
	"github.com/go-chi/cors"
	"gitlab.com/bloom42/bloom/common/consts"
	accountsrpc "gitlab.com/bloom42/bloom/common/rpc/accounts"
	billingrpc "gitlab.com/bloom42/bloom/common/rpc/billing"
	groupsrpc "gitlab.com/bloom42/bloom/common/rpc/groups"
	"gitlab.com/bloom42/bloom/server/config"
	accountshandler "gitlab.com/bloom42/bloom/server/domain/accounts/handler"
	billinghandler "gitlab.com/bloom42/bloom/server/domain/billing/handler"
	groupshandler "gitlab.com/bloom42/bloom/server/domain/groups/handler"
	"gitlab.com/bloom42/libs/rz-go"
	"gitlab.com/bloom42/libs/rz-go/log"
	"gitlab.com/bloom42/libs/rz-go/rzhttp"
)

func Run() error {
	var allowedOrigins []string
	router := chi.NewRouter()

	// replace size field name by latency and disable userAgent logging
	loggingMiddleware := rzhttp.Handler(log.Logger(), rzhttp.Duration("latency"))

	accountsHandler := accountsrpc.NewAccountsServer(accountshandler.Handler{}, nil)
	groupsHandler := groupsrpc.NewGroupsServer(groupshandler.Handler{}, nil)
	billingHandler := billingrpc.NewBillingServer(billinghandler.Handler{}, nil)

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
	if config.Env == consts.ENV_PRODUCTION {
		allowedOrigins = []string{"https://*.bloom.sh", "https://bloom.sh"}
	} else {
		allowedOrigins = []string{"*"}
	}
	cors := cors.New(cors.Options{
		AllowedOrigins:   allowedOrigins,
		AllowedMethods:   []string{"GET", "POST", "PUT", "DELETE", "OPTIONS"},
		AllowedHeaders:   []string{"Accept", "Authorization", "Content-Type", "Origin"},
		ExposedHeaders:   []string{},
		AllowCredentials: false,
		MaxAge:           3600,
	})
	router.Use(cors.Handler)
	if config.Env != consts.ENV_DEVELOPMENT {
		router.Use(SetSecurityHeadersMiddleware)
	}
	router.Use(SetContextMiddleware)

	router.Get("/", HelloWorlHandler)
	router.Mount(accountsHandler.PathPrefix(), accountsHandler)
	router.Mount(billingHandler.PathPrefix(), billingHandler)
	router.Mount(groupsHandler.PathPrefix(), groupsHandler)
	router.NotFound(http.HandlerFunc(NotFoundHandler))

	log.Info("starting server", rz.Uint16("port", config.Port))
	return http.ListenAndServe(fmt.Sprintf(":%d", config.Port), router)
}

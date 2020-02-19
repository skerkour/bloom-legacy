package api

import (
	"fmt"
	"net/http"

	"github.com/99designs/gqlgen/graphql/handler"
	"github.com/99designs/gqlgen/graphql/playground"
	"github.com/go-chi/chi"
	"github.com/go-chi/chi/middleware"
	"github.com/go-chi/cors"
	"gitlab.com/bloom42/bloom/common/consts"
	graphqlapi "gitlab.com/bloom42/bloom/server/api/graphql"
	"gitlab.com/bloom42/bloom/server/api/webhook"
	"gitlab.com/bloom42/bloom/server/config"
	"gitlab.com/bloom42/libs/rz-go"
	"gitlab.com/bloom42/libs/rz-go/log"
	"gitlab.com/bloom42/libs/rz-go/rzhttp"
)

func Run() error {
	var allowedOrigins []string
	router := chi.NewRouter()

	// replace size field name by latency and disable userAgent logging
	loggingMiddleware := rzhttp.Handler(log.Logger(), rzhttp.Duration("latency"))

	graphqlHandler := handler.NewDefaultServer(graphqlapi.NewExecutableSchema(graphqlapi.New()))

	/*
		router.Use(SetRequestID)
		router.Use(rzhttp.Handler(log.Logger()))
		router.Use(injectLoggerMiddleware(log.Logger()))
		router.Use(middleware.Recoverer)
		router.Use(middleware.Timeout(30 * time.Second))
	*/

	// here the order matters, otherwise loggingMiddleware won't see the request ID
	// router.Use(middleware.RealIP) commented because we will not use a reverse proxy
	router.Use(SetRequestIDMiddleware)
	router.Use(loggingMiddleware)
	router.Use(SetLoggerMiddleware(log.Logger()))
	router.Use(middleware.Recoverer)
	router.Use(middleware.RedirectSlashes)
	// router.Use(middleware.Timeout(60 * time.Second))
	if config.Env == consts.ENV_PRODUCTION {
		allowedOrigins = []string{"https://*.bloom.sh", config.WebsiteUrl, "https://bloom42.com"}
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
	router.Use(AuthMiddleware)

	// routes
	router.Get("/", IndexHandler)
	router.Route("/api", func(apiRouter chi.Router) {
		apiRouter.Get("/", HelloWorlHandler)

		apiRouter.Mount("/graphql", graphqlHandler)
		if config.Env == consts.ENV_DEVELOPMENT {
			apiRouter.Mount("/graphql/playground", playground.Handler("Bloom", "/api/graphql"))
		}

		apiRouter.Route("/webhooks", func(webhooksRouter chi.Router) {
			webhooksRouter.HandleFunc("/stripe", webhook.StripeHandler)
		})
	})
	router.NotFound(http.HandlerFunc(NotFoundHandler))

	log.Info("starting server", rz.Uint16("port", config.Server.Port))
	return http.ListenAndServe(fmt.Sprintf(":%d", config.Server.Port), router)
}

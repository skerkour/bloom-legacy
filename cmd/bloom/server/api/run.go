package api

import (
	"context"
	"crypto/tls"
	"fmt"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/99designs/gqlgen/graphql/handler"
	"github.com/99designs/gqlgen/graphql/playground"
	"github.com/go-chi/chi"
	"github.com/go-chi/chi/middleware"
	"github.com/go-chi/cors"
	graphqlapi "gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/webhook"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/config"
	"gitlab.com/bloom42/bloom/common/consts"
	"gitlab.com/bloom42/lily/rz"
	"gitlab.com/bloom42/lily/rz/log"
	"gitlab.com/bloom42/lily/rz/rzhttp"
	"golang.org/x/crypto/acme/autocert"
)

func Run() error {
	var allowedOrigins []string
	router := chi.NewRouter()
	var certManager *autocert.Manager
	var tlsConfig *tls.Config

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
	router.Use(middleware.Compress(5, "application/*", "text/*", "image/*"))
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

	if config.Server.HTTPPort != nil {
		log.Info("HTTPS requested. starting autocert")
		certManager = &autocert.Manager{
			Email:      config.Server.CertsEmail,
			Prompt:     autocert.AcceptTOS,
			HostPolicy: autocert.HostWhitelist(config.Server.Domains...),
			Cache:      autocert.DirCache(config.Server.CertsDirectory),
		}
		tlsConfig = &tls.Config{
			GetCertificate: certManager.GetCertificate,
			CipherSuites: []uint16{
				tls.TLS_AES_128_GCM_SHA256,
				tls.TLS_AES_256_GCM_SHA384,
				tls.TLS_CHACHA20_POLY1305_SHA256,
			},
			MinVersion:               tls.VersionTLS13,
			PreferServerCipherSuites: true,
		}
	}

	server := http.Server{
		Addr:         fmt.Sprintf(":%d", config.Server.Port),
		Handler:      router,
		ReadTimeout:  SERVER_READ_TIMEOUT,
		WriteTimeout: SERVER_WRITE_TIMEOUT,
		IdleTimeout:  SERVER_IDLE_TIMEOUT,
		TLSConfig:    tlsConfig,
	}

	log.Info("Starting server", rz.Uint16("port", config.Server.Port))
	go func() {
		var err error
		if config.Server.HTTPPort != nil {
			go func() {
				err := http.ListenAndServe(fmt.Sprintf(":%d", *config.Server.HTTPPort), certManager.HTTPHandler(nil))
				if err != nil {
					log.Fatal("listening HTTP", rz.Err(err))
				}
			}()
			err = server.ListenAndServeTLS("", "") // Key and cert are coming from Let's Encrypt
		} else {
			err = server.ListenAndServe()
		}
		if err != nil {
			log.Fatal("listening", rz.Err(err))
		}
	}()

	signalCatcher := make(chan os.Signal, 1)

	signal.Notify(signalCatcher, os.Interrupt,
		syscall.SIGHUP,
		syscall.SIGINT,
		syscall.SIGTERM,
		syscall.SIGQUIT)
	sig := <-signalCatcher
	log.Info("Server is shutting down", rz.String("reason", sig.String()))

	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()

	server.SetKeepAlivesEnabled(false)
	if err := server.Shutdown(ctx); err != nil {
		log.Fatal("Could not gracefuly shutdown the server", rz.Err(err))
	}
	log.Info("Server stopped")
	return nil
}

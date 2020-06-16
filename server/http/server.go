package http

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
	"gitlab.com/bloom42/bloom/common/consts"
	"gitlab.com/bloom42/bloom/server/app/config"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/domain/sync"
	"gitlab.com/bloom42/bloom/server/domain/users"
	graphqlapi "gitlab.com/bloom42/bloom/server/server/api/graphql"
	"gitlab.com/bloom42/bloom/server/server/api/webhook"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/log/loghttp"
	"gitlab.com/bloom42/gobox/rz"
	"golang.org/x/crypto/acme/autocert"
)

// Server represents an http server
type Server struct {
	usersService   users.Service
	groupsService  groups.Service
	syncService    sync.Service
	billingService billing.Service
	config         config.Config
	logger         log.Logger
	router         *chi.Mux
	httpServer     http.Server
}

// NewServer returns a new, configured instance of `Server`
func NewServer(conf config.Config, logger log.Logger, usersService users.Service, groupsService groups.Service,
	syncService sync.Service, billingService billing.Service) *Server {
	server := Server{
		usersService:   usersService,
		groupsService:  groupsService,
		syncService:    syncService,
		billingService: billingService,
		config:         conf,
		logger:         logger,
		router:         chi.NewRouter(),
	}

	return &server
}

// Run run the HTTP server
func (server *Server) Run() error {
	var allowedOrigins []string
	var certManager *autocert.Manager
	var tlsConfig *tls.Config
	var serverAddress string

	// replace size field name by latency and disable userAgent logging
	loggingMiddleware := loghttp.Handler(logger, loghttp.Duration("latency"))

	// setup middlewares

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

	// setup routes
	graphqlHandler := handler.NewDefaultServer(graphqlapi.NewExecutableSchema(graphqlapi.New()))
	webhookAPI := webhook.NewAPI(server.billingService)

	router.Get("/", IndexHandler)
	router.Route("/api", func(apiRouter chi.Router) {
		apiRouter.Get("/", HelloWorlHandler)

		apiRouter.Mount("/graphql", graphqlHandler)
		if config.Env == consts.ENV_DEVELOPMENT {
			apiRouter.Mount("/graphql/playground", playground.Handler("Bloom", "/api/graphql"))
		}

		apiRouter.Route("/webhooks", func(webhooksRouter chi.Router) {
			webhooksRouter.HandleFunc("/stripe", webhookAPI.StripeHandler)
		})
	})
	router.NotFound(http.HandlerFunc(server.HandlerNotFound))

	if config.Server.HTTPSPort != nil {
		server.logger.Info("HTTPS requested. starting autocert")
		certManager = &autocert.Manager{
			Email:      config.Server.CertsEmail,
			Prompt:     autocert.AcceptTOS,
			HostPolicy: autocert.HostWhitelist(config.Server.Domains...),
			Cache:      autocert.DirCache(config.Server.CertsDirectory),
		}
		tlsConfig = &tls.Config{
			GetCertificate: certManager.GetCertificate,
			// Only use curves which have assembly implementations
			CurvePreferences: []tls.CurveID{
				tls.X25519,
			},
			CipherSuites: []uint16{
				tls.TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384,
				tls.TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305,
				tls.TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256,
			},
			MinVersion:               tls.VersionTLS13,
			PreferServerCipherSuites: true,
		}
		serverAddress = fmt.Sprintf(":%d", *config.Server.HTTPSPort)
	} else {
		serverAddress = fmt.Sprintf(":%d", config.Server.HTTPPort)
	}

	httpServer := http.Server{
		Addr:         serverAddress,
		Handler:      router,
		ReadTimeout:  SERVER_READ_TIMEOUT,
		WriteTimeout: SERVER_WRITE_TIMEOUT,
		IdleTimeout:  SERVER_IDLE_TIMEOUT,
		TLSConfig:    tlsConfig,
	}

	server.logger.Info("Starting server", log.Uint16("http_port", config.Server.HTTPPort))
	go func() {
		var err error
		if config.Server.HTTPSPort != nil {
			go func() {
				httpServerAddress := fmt.Sprintf(":%d", config.Server.HTTPPort)
				err := http.ListenAndServe(httpServerAddress, certManager.HTTPHandler(nil))
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

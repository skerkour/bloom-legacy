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
	"gitlab.com/bloom42/bloom/server/api/graphql"
	"gitlab.com/bloom42/bloom/server/api/webhook"
	"gitlab.com/bloom42/bloom/server/app/config"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/domain/sync"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/log/loghttp"
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
	httpServer     http.Server
	certManager    *autocert.Manager
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
	}
	var allowedOrigins []string
	var tlsConfig *tls.Config
	var serverAddress string
	router := chi.NewRouter()

	// setup middlewares

	// replace size field name by latency and disable userAgent logging
	loggingMiddleware := loghttp.Handler(logger, loghttp.Duration("latency"))
	if server.config.Env == config.EnvProduction {
		allowedOrigins = []string{"https://*.bloom.sh", server.config.WebsiteURL, "https://bloom42.com"}
	} else {
		allowedOrigins = []string{"*"}
	}
	cors := cors.New(cors.Options{
		AllowedOrigins:   allowedOrigins,
		AllowedMethods:   []string{"GET", "POST", "PUT", "DELETE", "OPTIONS"},
		AllowedHeaders:   []string{"Accept", "Authorization", "Content-Type", "Origin"},
		ExposedHeaders:   []string{},
		AllowCredentials: false,
		MaxAge:           CORSMaxAge,
	})
	router.Use(middleware.Compress(5, "application/*", "text/*", "image/*"))
	router.Use(server.MiddlewareSetRequestID)
	router.Use(loggingMiddleware)
	router.Use(server.MiddlewareSetLogger)
	router.Use(middleware.Recoverer)
	router.Use(middleware.RedirectSlashes)
	router.Use(server.MiddlewareSetHTTPContext)
	router.Use(server.MiddlewareAuth)
	// router.Use(middleware.Timeout(60 * time.Second))
	router.Use(cors.Handler)
	if server.config.Env != config.EnvDevelopment {
		router.Use(server.MiddlewareSetSecurityHeaders)
	}

	// setup routes
	graphqlHandler := handler.NewDefaultServer(graphql.NewExecutableSchema(graphql.NewAPI(
		server.config,
		server.usersService,
		server.groupsService,
		server.syncService,
		server.billingService,
	)))
	webhookAPI := webhook.NewAPI(server.config, server.billingService)

	router.Get("/", server.HandlerIndex)
	router.Route("/api", func(apiRouter chi.Router) {
		apiRouter.Get("/", server.HandlerHelloWorld)

		apiRouter.Mount("/graphql", graphqlHandler)
		if server.config.Env == config.EnvDevelopment {
			apiRouter.Mount("/graphql/playground", playground.Handler("Bloom", "/api/graphql"))
		}

		apiRouter.Route("/webhooks", func(webhooksRouter chi.Router) {
			webhooksRouter.HandleFunc("/stripe", webhookAPI.StripeHandler)
		})
	})
	router.NotFound(http.HandlerFunc(server.HandlerNotFound))

	if server.config.HTTP.HTTPSPort != nil {
		server.logger.Info("HTTPS requested. starting autocert")
		server.certManager = &autocert.Manager{
			Email:      server.config.HTTP.CertsEmail,
			Prompt:     autocert.AcceptTOS,
			HostPolicy: autocert.HostWhitelist(server.config.HTTP.Domains...),
			Cache:      autocert.DirCache(server.config.HTTP.CertsDirectory),
		}
		tlsConfig = &tls.Config{
			GetCertificate: server.certManager.GetCertificate,
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
		serverAddress = fmt.Sprintf(":%d", *server.config.HTTP.HTTPSPort)
	} else {
		serverAddress = fmt.Sprintf(":%d", server.config.HTTP.Port)
	}

	server.httpServer = http.Server{
		Addr:         serverAddress,
		Handler:      router,
		ReadTimeout:  ReadTimeout,
		WriteTimeout: WriteTimeout,
		IdleTimeout:  IdleTimeout,
		TLSConfig:    tlsConfig,
	}

	return &server
}

// Run run the HTTP server
func (server *Server) Run() error {
	server.logger.Info("Starting server", log.Uint16("http_port", server.config.HTTP.Port))
	go func() {
		var err error
		if server.config.HTTP.HTTPSPort != nil {
			go func() {
				httpServerAddress := fmt.Sprintf(":%d", server.config.HTTP.Port)
				err := http.ListenAndServe(httpServerAddress, server.certManager.HTTPHandler(nil))
				if err != nil {
					server.logger.Fatal("http.Run: listening HTTP", log.Err("error", err))
				}
			}()
			err = server.httpServer.ListenAndServeTLS("", "") // Key and cert are coming from Let's Encrypt
		} else {
			err = server.httpServer.ListenAndServe()
		}
		if err != nil {
			server.logger.Fatal("http.Run: listening", log.Err("error", err))
		}
	}()

	signalCatcher := make(chan os.Signal, 1)

	signal.Notify(signalCatcher, os.Interrupt,
		syscall.SIGHUP,
		syscall.SIGINT,
		syscall.SIGTERM,
		syscall.SIGQUIT)
	sig := <-signalCatcher
	server.logger.Info("http.Run: Server is shutting down", log.String("reason", sig.String()))

	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()

	server.httpServer.SetKeepAlivesEnabled(false)
	if err := server.httpServer.Shutdown(ctx); err != nil {
		server.logger.Fatal("http.Run: Could not gracefuly shutdown the server", log.Err("error", err))
	}
	server.logger.Info("http.Run: Server stopped")
	return nil
}

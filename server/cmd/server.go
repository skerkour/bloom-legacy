package cmd

import (
	"fmt"
	"net/http"

	"github.com/go-chi/chi"
	"github.com/spf13/cobra"
	rpcaccounts "gitlab.com/bloom42/bloom/core/rpc/accounts"
	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/bloom/accounts"
	"gitlab.com/bloom42/bloom/server/config"
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
		err := config.Init(configFileFlag)
		if err != nil {
			log.Fatal("Initializing config", rz.Err(err))
		}

		log.SetLogger(log.With(
			rz.Fields(
				rz.String("service", "api"), rz.String("host", "abcd.local"), rz.String("env", config.Config.Env),
			),
		))

		router := chi.NewRouter()

		// replace size field name by latency and disable userAgent logging
		loggingMiddleware := rzhttp.Handler(log.Logger(), rzhttp.Duration("latency"))

		accountsHandler := rpcaccounts.NewAccountsServer(accounts.Handler{}, nil)

		// here the order matters, otherwise loggingMiddleware won't see the request ID
		router.Use(api.SecurityHeadersMiddleware)
		router.Use(api.SetRequestIDMiddleware)
		router.Use(loggingMiddleware)
		router.Use(api.SetLoggerMiddleware(log.Logger()))

		router.Get("/", api.HelloWorlHandler)
		router.Mount(accountsHandler.PathPrefix(), accountsHandler)
		router.NotFound(http.HandlerFunc(api.NotFoundHandler))

		log.Info("starting server", rz.Uint16("port", config.Config.Port))
		err = http.ListenAndServe(fmt.Sprintf(":%d", config.Config.Port), router)
		if err != nil {
			log.Fatal("listening", rz.Err(err))
		}
	},
}

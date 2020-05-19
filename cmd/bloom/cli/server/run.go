package server

import (
	"github.com/getsentry/sentry-go"
	"github.com/spf13/cobra"
	"github.com/stripe/stripe-go"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/config"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/db"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/services/notification"
	"gitlab.com/bloom42/bloom/common/consts"
	"gitlab.com/bloom42/gobox/rz"
	"gitlab.com/bloom42/gobox/rz/log"
)

func init() {
	ServerCmd.AddCommand(runCmd)
}

var runCmd = &cobra.Command{
	Use:     "run",
	Aliases: []string{"r"},
	Short:   "Run the server",
	Long:    "Run the server",
	Run: func(cmd *cobra.Command, args []string) {
		err := config.Init(configFileFlag)
		if err != nil {
			log.Fatal("Initializing config", rz.Err(err))
		}

		// init internal services
		log.SetLogger(log.With(
			rz.Fields(
				rz.String("service", "bloom_api"), rz.String("host", "abcd.local"), rz.String("env", config.Env),
			),
		))

		if config.Env == consts.ENV_DEVELOPMENT {
			log.SetLogger(log.With(rz.Formatter(rz.FormatterConsole())))
		} else {
			log.SetLogger(log.With(rz.Formatter(nil))) // JSON logging
			stripe.DefaultLeveledLogger = &stripe.LeveledLogger{
				Level: stripe.LevelInfo,
			}
		}

		// init 3rd party services
		stripe.Key = config.Stripe.SecretKey
		err = sentry.Init(sentry.ClientOptions{
			Dsn: config.Sentry.Dsn,
		})
		if err != nil {
			log.Fatal("Initializing Sentry", rz.Err(err))
		}

		err = notification.Init(config.SMTP)
		if err != nil {
			log.Fatal("Initializing noitification", rz.Err(err))
		}

		err = db.Init()
		if err != nil {
			log.Fatal("Connecting to database", rz.Err(err))
		}
		log.Info("Successfully connected to database")

		err = users.InitGlobalSessionsCache(log.Logger())
		if err != nil {
			log.Fatal("Initalizing sessions cache", rz.Err(err))
		}
		log.Info("sessions cache Successfully initialized")

		err = api.Run()
		if err != nil {
			log.Fatal("listening", rz.Err(err))
		}
	},
}

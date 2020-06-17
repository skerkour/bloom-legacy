package cli

import (
	"fmt"

	"github.com/getsentry/sentry-go"
	"github.com/stripe/stripe-go"
	"gitlab.com/bloom42/bloom/server/app/config"
	"gitlab.com/bloom42/bloom/server/db"
	billingrepository "gitlab.com/bloom42/bloom/server/domain/billing/repository"
	billingservice "gitlab.com/bloom42/bloom/server/domain/billing/service"
	groupsrepository "gitlab.com/bloom42/bloom/server/domain/groups/repository"
	groupservice "gitlab.com/bloom42/bloom/server/domain/groups/service"
	syncrepository "gitlab.com/bloom42/bloom/server/domain/sync/repository"
	syncservice "gitlab.com/bloom42/bloom/server/domain/sync/service"
	usersrepository "gitlab.com/bloom42/bloom/server/domain/users/repository"
	usersservice "gitlab.com/bloom42/bloom/server/domain/users/service"
	"gitlab.com/bloom42/bloom/server/driver/cache/inmemory"
	"gitlab.com/bloom42/bloom/server/driver/mailer/smtp"
	"gitlab.com/bloom42/bloom/server/http"
	"gitlab.com/bloom42/gobox/cli"
	"gitlab.com/bloom42/gobox/log"
)

var serverCmd = &cli.Command{
	Use:     "server",
	Aliases: []string{"s", "serve"},
	Short:   "Run the server",
	RunE: func(cmd *cli.Command, args []string) error {
		conf, err := config.Load(configFileFlag)
		if err != nil {
			return err
		}

		logger := log.GlobalLogger().Clone(
			log.SetFields(log.String("env", conf.Env)),
			log.SetLevel(log.DebugLevel),
		)

		if conf.Env != config.EnvDevelopment {
			logger = logger.Clone(log.SetFormatter(nil), log.SetLevel(log.InfoLevel)) // JSON logging
			stripe.DefaultLeveledLogger = &stripe.LeveledLogger{
				Level: stripe.LevelInfo,
			}
		}

		// init 3rd party services
		stripe.Key = conf.Stripe.SecretKey
		err = sentry.Init(sentry.ClientOptions{
			Dsn: conf.Sentry.Dsn,
		})
		if err != nil {
			return fmt.Errorf("Initializing Sentry: %w", err)
		}

		mailer := smtp.NewMailer(conf.SMTP)
		cache := inmemory.NewCache()
		db, err := db.Connect(conf.Database.URL, conf.Database.PoolSize)
		if err != nil {
			return err
		}

		usersRepo := usersrepository.NewUsersRepository(cache)
		billingRepo := billingrepository.NewBillingRepository()
		groupsRepo := groupsrepository.NewGroupsRepository()
		syncRepo := syncrepository.NewSyncRepository()

		billingService := billingservice.NewBillingService(db, billingRepo, mailer)
		usersService := usersservice.NewUsersService(db, usersRepo, billingService, mailer)
		groupsService := groupservice.NewGroupsService(db)
		billingService.InjectUsersService(usersService)
		billingService.InjectGroupsService(groupsService)
		syncService := syncservice.NewSyncService(db, usersService, groupsService, syncRepo,
			groupsRepo, billingRepo, billingService, usersRepo)

		server := http.NewServer(
			conf,
			logger,
			usersService,
			groupsService,
			syncService,
			billingService,
		)

		return server.Run()
	},
}

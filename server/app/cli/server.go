package cli

import (
	"gitlab.com/bloom42/gobox/cli"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/megabox/server/app/config"
	"gitlab.com/bloom42/megabox/server/db"
	usersrepository "gitlab.com/bloom42/megabox/server/domain/users/repository"
	usersservice "gitlab.com/bloom42/megabox/server/domain/users/service"
	"gitlab.com/bloom42/megabox/server/driver/cache/inmemory"
	"gitlab.com/bloom42/megabox/server/driver/mailer/smtp"
	"gitlab.com/bloom42/megabox/server/http"
)

var runCmd = &cli.Command{
	Use:     "server",
	Aliases: []string{"s", "serve"},
	Short:   "Run the server",
	RunE: func(cmd *cli.Command, args []string) error {
		conf, err := config.Load(configFileFlag)
		if err != nil {
			return err
		}

		if conf.Env == config.EnvDevelopment {
			log.SetGlobalLogger(log.Clone(log.SetLevel(log.DebugLevel)))
		} else {
			log.SetGlobalLogger(log.Clone(log.SetFormatter(nil), log.SetLevel(log.InfoLevel))) // JSON logging
		}

		mailer := smtp.NewMailer(conf.SMTP)
		logger := log.GlobalLogger().Clone()
		cache := inmemory.NewCache()
		db, err := db.Connect(conf.Database.URL, conf.Database.PoolSize)
		if err != nil {
			return err
		}

		usersRepo := usersrepository.NewUsersRepository(cache)
		usersService := usersservice.NewUsersService(db, usersRepo, mailer)
		server := http.NewServer(conf, logger, usersService)

		return server.Run()
	},
}

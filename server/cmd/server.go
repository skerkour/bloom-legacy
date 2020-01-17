package cmd

import (
	"github.com/spf13/cobra"
	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/config"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/services/notification"
	"gitlab.com/bloom42/bloom/server/services/util"
	"gitlab.com/bloom42/libs/rz-go"
	"gitlab.com/bloom42/libs/rz-go/log"
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
				rz.String("service", "api"), rz.String("host", "abcd.local"), rz.String("env", config.Env),
			),
		))

		err = util.Init()
		if err != nil {
			log.Fatal("Initializing util", rz.Err(err))
		}

		err = notification.Init()
		if err != nil {
			log.Fatal("Initializing noitification", rz.Err(err))
		}

		err = db.Init()
		if err != nil {
			log.Fatal("Connecting to database", rz.Err(err))
		}
		log.Info("Successfully connected to database")

		err = api.Run()
		if err != nil {
			log.Fatal("listening", rz.Err(err))
		}
	},
}

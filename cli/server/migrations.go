package server

import (
	"github.com/golang-migrate/migrate/v4"
	_ "github.com/golang-migrate/migrate/v4/database/postgres"
	_ "github.com/golang-migrate/migrate/v4/source/file"
	"github.com/spf13/cobra"
	"gitlab.com/bloom42/bloom/server/config"
	"gitlab.com/bloom42/libs/rz-go"
	"gitlab.com/bloom42/libs/rz-go/log"
)

var singleMigrationFlag bool

func init() {
	migrationsCmd.PersistentFlags().BoolVarP(&singleMigrationFlag, "single", "s", false, "Run or revert only one migrations")
	migrationsCmd.AddCommand(migrationsRunCmd)
	migrationsCmd.AddCommand(migrationsRevertCmd)
	ServerCmd.AddCommand(migrationsCmd)
}

// migrationsCmd is the bloomserver's `version` command. It display various information about the current phaser executable
var migrationsCmd = &cobra.Command{
	Use:     "migrations",
	Aliases: []string{"m"},
	Short:   "Manage database migrations",
	Run: func(cmd *cobra.Command, args []string) {
		cmd.Help()
	},
}

var migrationsRunCmd = &cobra.Command{
	Use:   "run",
	Short: "Run all pending migration",
	Run: func(cmd *cobra.Command, args []string) {
		err := config.Init(configFileFlag)
		if err != nil {
			log.Fatal("Initializing config", rz.Err(err))
		}

		migrate, err := migrate.New("file://migrations", config.Database.URL)

		if err != nil {
			log.Fatal("Initializing DB connection", rz.Err(err))
		}

		if singleMigrationFlag {
			err = migrate.Steps(1)
		} else {
			err = migrate.Up()
		}

		if err != nil {
			log.Fatal("Running migrations", rz.Err(err))
		}
	},
}

var migrationsRevertCmd = &cobra.Command{
	Use:     "revert",
	Aliases: []string{"rev"},
	Short:   "Revert the last migration",
	Run: func(cmd *cobra.Command, args []string) {
		err := config.Init(configFileFlag)
		if err != nil {
			log.Fatal("Initiating config", rz.Err(err))
		}

		migrate, err := migrate.New("file://migrations", config.Database.URL)

		if err != nil {
			log.Fatal("Initializing DB connection", rz.Err(err))
		}

		if singleMigrationFlag {
			err = migrate.Steps(-1)
		} else {
			err = migrate.Down()
		}
		if err != nil {
			log.Fatal("Reverting migrations", rz.Err(err))
		}
	},
}

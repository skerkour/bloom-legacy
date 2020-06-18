package cli

import (
	"fmt"

	"github.com/golang-migrate/migrate/v4"
	// import postgres driver for migration
	_ "github.com/golang-migrate/migrate/v4/database/postgres"
	_ "github.com/golang-migrate/migrate/v4/source/file"
	"gitlab.com/bloom42/bloom/server/app/config"
	"gitlab.com/bloom42/gobox/cli"
)

var revertAllMigrationsFlag bool
var migrationsDirPath string

func init() {
	migrationsRevertCmd.PersistentFlags().BoolVarP(&revertAllMigrationsFlag, "all", "a", false, "Revert all migrations")
	migrationsCmd.AddCommand(migrationsRunCmd)
	migrationsCmd.AddCommand(migrationsRevertCmd)
	migrationsDirPath = fmt.Sprintf("file://%s", config.MigrationsDir)
}

var migrationsCmd = &cli.Command{
	Use:     "migrations",
	Aliases: []string{"m"},
	Short:   "Manage database migrations",
	Run: func(cmd *cli.Command, args []string) {
		cmd.Help()
	},
}

var migrationsRunCmd = &cli.Command{
	Use:   "run",
	Short: "Run all pending migration",
	RunE: func(cmd *cli.Command, args []string) error {
		conf, err := config.Load(configFileFlag)
		if err != nil {
			return err
		}

		migrate, err := migrate.New(migrationsDirPath, conf.Database.URL)
		if err != nil {
			return err
		}

		err = migrate.Up()
		return err
	},
}

var migrationsRevertCmd = &cli.Command{
	Use:     "revert",
	Aliases: []string{"rev"},
	Short:   "Revert the last migration",
	RunE: func(cmd *cli.Command, args []string) error {
		conf, err := config.Load(configFileFlag)
		if err != nil {
			return err
		}

		migrate, err := migrate.New(migrationsDirPath, conf.Database.URL)
		if err != nil {
			return err
		}

		if revertAllMigrationsFlag {
			err = migrate.Down()
		} else {
			err = migrate.Steps(-1)
		}
		return err
	},
}

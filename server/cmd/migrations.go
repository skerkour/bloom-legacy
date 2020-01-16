package cmd

import (
	"github.com/golang-migrate/migrate/v4"
	_ "github.com/golang-migrate/migrate/v4/database/postgres"
	"github.com/spf13/cobra"
	"gitlab.com/bloom42/libs/rz-go"
	"gitlab.com/bloom42/libs/rz-go/log"
)

// migrationsCmd is the bloomserver's `version` command. It display various information about the current phaser executable
var migrationsCmd = &cobra.Command{
	Use:     "migrations",
	Aliases: []string{"m"},
	Short:   "Manage database migrations",
	Long:    "Manage database migrations",
	Run: func(cmd *cobra.Command, args []string) {
		migrate, err := migrate.New(
			"file:///migrations",
			"postgres://localhost:5432/database?sslmode=enable")

		if err != nil {
			log.Fatal("Initiating DB connection", rz.Err(err))
		}

		err = migrate.Up()
		if err != nil {
			log.Fatal("Running migrations", rz.Err(err))
		}
	},
}

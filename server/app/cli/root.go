package cli

import (
	"gitlab.com/bloom42/bloom/server/app"
	"gitlab.com/bloom42/gobox/cli"
)

var configFileFlag string

func init() {
	rootCmd.PersistentFlags().StringVarP(&configFileFlag, "config", "c", app.DefaultConfigPath, "Configuration file path")

	rootCmd.AddCommand(versionCmd)
	rootCmd.AddCommand(serverCmd)
	rootCmd.AddCommand(migrationsCmd)
	rootCmd.AddCommand(initCmd)
}

var rootCmd = &cli.Command{
	Use:   "bloom",
	Short: "Bloom. Visit https://bloom.sh for more information",
	RunE: func(cmd *cli.Command, args []string) error {
		return cmd.Help()
	},
}

// Execute runs rootCmd and returns its result
func Execute() error {
	return rootCmd.Execute()
}

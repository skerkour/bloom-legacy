package cli

import (
	"gitlab.com/bloom42/gobox/cli"
	"gitlab.com/bloom42/megabox/server/app"
)

var configFileFlag string

func init() {
	rootCmd.PersistentFlags().StringVarP(&configFileFlag, "config", "c", app.DefaultConfigPath, "Configuration file path")

	rootCmd.AddCommand(versionCmd)
	rootCmd.AddCommand(runCmd)
	rootCmd.AddCommand(migrationsCmd)
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

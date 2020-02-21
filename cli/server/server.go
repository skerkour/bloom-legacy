package server

import (
	"github.com/spf13/cobra"
)

var configFileFlag string

func init() {
	ServerCmd.PersistentFlags().StringVarP(&configFileFlag, "config", "c", "bloom.sane", "Configuration file")
}

var ServerCmd = &cobra.Command{
	Use:   "server",
	Short: "Bloom server",
	Long:  "All commands related to the Bloom server. Visit https://bloom.sh for more information.",
	Run: func(cmd *cobra.Command, args []string) {
		cmd.Help()
	},
}

func Execute() error {
	return ServerCmd.Execute()
}

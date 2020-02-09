package cli

import (
	"github.com/spf13/cobra"
)

var configFileFlag string

func init() {
	rootCmd.PersistentFlags().StringVarP(&configFileFlag, "cofnig", "c", "bloom.sane", "Configuration file")
}

var rootCmd = &cobra.Command{
	Use:   "bloomserver",
	Short: "Bloom server",
	Long:  "Bloom server. Visit https://bloom.sh for more information",
	Run: func(cmd *cobra.Command, args []string) {
		cmd.Help()
	},
}

func Execute() error {
	return rootCmd.Execute()
}

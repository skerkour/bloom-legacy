package cmd

import (
	"github.com/spf13/cobra"
	"gitlab.com/bloom42/bloom/bitflow/worker"
	"gitlab.com/bloom42/libs/rz-go/log"
)

func init() {
	rootCmd.AddCommand(workerCmd)
}

// run the scanner as a worker, waiting messages from remote sources
var workerCmd = &cobra.Command{
	Use:     "worker",
	Aliases: []string{"w"},
	Short:   "Run bitflow as a worker",
	Long:    "Run bitflow as a worker waiting for messages from remote sources. Configuration is done with environment variable",
	Run: func(cmd *cobra.Command, args []string) {
		var w worker.Worker

		if err := w.Run(); err != nil {
			log.Fatal(err.Error())
		}
	},
}

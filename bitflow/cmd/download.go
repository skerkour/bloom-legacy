package cmd

import (
	"fmt"
	"strings"

	"github.com/spf13/cobra"
	"gitlab.com/bloom42/bloom/bitflow/dl"
	"gitlab.com/bloom42/libs/rz-go"
	"gitlab.com/bloom42/libs/rz-go/log"
)

func init() {
	rootCmd.AddCommand(downloadCmd)
}

var downloadCmd = &cobra.Command{
	Use:     "download [.torrent file or magnet link]",
	Aliases: []string{"dl", "d"},
	Short:   "Downlaod a Torrent",
	Args:    cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		log.SetLogger(log.With(
			rz.Formatter(rz.FormatterCLI()),
			rz.Level(rz.InfoLevel),
		))
		var err error
		var download dl.Download

		// basic detection if argument is a .torrent file
		if strings.HasSuffix(args[0], ".torrent") {
			download, err = dl.Torrent(args[0], "")
		} else {
			download, err = dl.Magnet(args[0], "")
		}

		if err != nil {
			log.Fatal(err.Error())
		}
		log.Info(fmt.Sprintf("%s successfully downloaded in %s", download.Name, download.Path))
	},
}

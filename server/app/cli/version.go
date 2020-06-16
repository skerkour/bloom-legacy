package cli

import (
	"encoding/json"
	"fmt"

	"gitlab.com/bloom42/gobox/cli"
	"gitlab.com/bloom42/megabox/server/app"
)

type versionJSON struct {
	Name         string `json:"name"`
	Version      string `json:"version"`
	GitCommit    string `json:"gitCommit"`
	UTCBuildTime string `json:"UTCBuildTime"`
	OS           string `json:"os"`
	Architecture string `json:"architecture"`
	GoVersion    string `json:"goVersion"`
}

var versionOutputFormat string

func init() {
	versionCmd.Flags().StringVarP(&versionOutputFormat, "format", "f", "text", "The ouput format. Valid values are [text, json]")
}

// versionCmd is the server's `version` command. It display various information about the current phaser executable
var versionCmd = &cli.Command{
	Use:     "version",
	Aliases: []string{"v"},
	Short:   "Display the version and build information",
	Long:    "Display the version and build information",
	RunE: func(cmd *cli.Command, args []string) error {
		var err error

		switch versionOutputFormat {
		case "text":
			renderVersionText()
		case "json":
			err = renderVersionJSON()
		default:
			err = fmt.Errorf("%s is not a valid output format", versionOutputFormat)
		}
		return err
	},
}

func renderVersionText() {
	fmt.Printf("Name           : %s\n", app.Name)
	fmt.Printf("Version        : %s\n", app.Version)
	fmt.Printf("Git commit     : %s\n", app.GitCommit)
	fmt.Printf("UTC build time : %s\n", app.UTCBuildTime)
	fmt.Printf("OS             : %s\n", app.OS)
	fmt.Printf("Architecture   : %s\n", app.Arch)
	fmt.Printf("Go version     : %s\n", app.GoVersion)
}

func renderVersionJSON() error {
	var err error
	var output []byte

	jsonVersion := versionJSON{
		Name:         app.Name,
		Version:      app.Version,
		GitCommit:    app.GitCommit,
		UTCBuildTime: app.UTCBuildTime,
		OS:           app.OS,
		Architecture: app.Arch,
		GoVersion:    app.GoVersion,
	}
	output, err = json.Marshal(&jsonVersion)
	if err == nil {
		fmt.Println(string(output))
	}
	return err
}

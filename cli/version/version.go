package version

import (
	"encoding/json"
	"fmt"

	"github.com/spf13/cobra"
	"gitlab.com/bloom42/bloom/version"
	"gitlab.com/bloom42/libs/rz-go/log"
)

type versionJSON struct {
	Name         string `json:"name"`
	Version      string `json:"version"`
	GitCommit    string `json:"git_commit"`
	UTCBuildTime string `json:"utc_build_time"`
	OS           string `json:"os"`
	Architecture string `json:"architecture"`
	GoVersion    string `json:"go_version"`
}

var versionOutputFormat string

func init() {
	VersionCmd.Flags().StringVarP(&versionOutputFormat, "format", "f", "text", "The ouput format. Valid values are [text, json]")
}

// VersionCmd is the bloom `version` command. It display various information about the current phaser executable
var VersionCmd = &cobra.Command{
	Use:     "version",
	Aliases: []string{"v"},
	Short:   "Display the version and build information",
	Long:    "Display the version and build information",
	Run: func(cmd *cobra.Command, args []string) {
		var err error

		switch versionOutputFormat {
		case "text":
			renderVersionText()
		case "json":
			err = renderVersionJSON()
		default:
			err = fmt.Errorf("%s is not a valid output format", versionOutputFormat)
		}
		if err != nil {
			log.Fatal(err.Error())
		}
	},
}

func renderVersionText() {
	fmt.Printf("Name           : %s\n", version.Name)
	fmt.Printf("Version        : %s\n", version.Version)
	fmt.Printf("Git commit     : %s\n", version.GitCommit)
	fmt.Printf("UTC build time : %s\n", version.UTCBuildTime)
	fmt.Printf("OS             : %s\n", version.OS)
	fmt.Printf("Architecture   : %s\n", version.Arch)
	fmt.Printf("Go version     : %s\n", version.GoVersion)
}

func renderVersionJSON() error {
	var err error
	var output []byte

	jsonVersion := versionJSON{
		Name:         version.Name,
		Version:      version.Version,
		GitCommit:    version.GitCommit,
		UTCBuildTime: version.UTCBuildTime,
		OS:           version.OS,
		Architecture: version.Arch,
		GoVersion:    version.GoVersion,
	}
	output, err = json.Marshal(&jsonVersion)
	if err == nil {
		fmt.Println(string(output))
	}
	return err
}

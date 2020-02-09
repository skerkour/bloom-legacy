package version

import (
	"runtime"
)

// const set at build time

var GitCommit string
var UTCBuildTime string
var GoVersion string

const (
	OS      = runtime.GOOS
	Arch    = runtime.GOARCH
	Name    = "bloom"
	Version = "0.1.0"
)

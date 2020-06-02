package version

import (
	"runtime"
)

// set at build time
var GitCommit string
var UTCBuildTime string
var GoVersion string

const (
	OS   = runtime.GOOS
	Arch = runtime.GOARCH
	Name = "bitflow"
	// Version is bitflow's version
	Version = "0.7.1"
)

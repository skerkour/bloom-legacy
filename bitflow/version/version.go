package version

import (
	"runtime"
)

// const set at build time
const (
	UTCBuildTime = "undefined"
	GitCommit    = "undefined"
	OS           = runtime.GOOS
	Arch         = runtime.GOARCH
	GoVersion    = "undefined"
	Name         = "bitflow"
)

// Version is the bitflow's version
const (
	Version = "0.7.1"
)

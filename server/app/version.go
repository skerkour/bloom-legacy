package app

import (
	"runtime"
)

// GitCommit is set at build time
var GitCommit string

// UTCBuildTime is set at build time
var UTCBuildTime string

// GoVersion is the go version used to compile the program, set at build time
var GoVersion string

const (
	// OS is the OS the program is run on
	OS = runtime.GOOS
	// Arch is the processor architecture the program is run on
	Arch = runtime.GOARCH
	// Name is the name of the program
	Name = "bloom"
	// Version is the version of the program
	Version = "1.0.0"
)

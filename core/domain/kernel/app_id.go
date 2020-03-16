package kernel

import (
	"runtime"
)

// AppID returns the app's ID
func AppID() string {
	if runtime.GOOS == "android" {
		// for legacy reasons...
		return "com.bloom42.bloomx"
	}
	return "com.bloom42.bloom"
}

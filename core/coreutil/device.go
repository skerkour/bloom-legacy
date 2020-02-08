package coreutil

import (
	"runtime"
)

// GetDeviceOS return the device's OS, formatted for the Bloom api
func GetDeviceOS() string {
	switch runtime.GOOS {
	case "android":
		return "ANDROID"
	case "windows":
		return "WINDOWS"
	case "ios":
		return "IOS"
	case "darwin":
		arch := runtime.GOARCH
		if arch == "arm" || arch == "arm64" {
			return "IOS"
		} else {
			return "MACOS"
		}
	default:
		return "linux"
	}
}

// GetDeviceType return the device's type, formatted for the Bloom api
func GetDeviceType() string {
	switch GetDeviceOS() {
	case "ios", "android":
		return "MOBILE"
	default:
		return "COMPUTER"
	}
}

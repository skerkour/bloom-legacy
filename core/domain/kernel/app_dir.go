package kernel

import (
	"bytes"
	"io/ioutil"
	"os"
	"path/filepath"
	"runtime"
)

// appDir is used to cache AppDir
var appDir string

// AppDir returns the directory which should be used to store user data
func AppDir() (string, error) {
	if appDir != "" {
		return appDir, nil
	}

	if runtime.GOOS == "android" {
		data, err := ioutil.ReadFile("/proc/self/cmdline")
		if err != nil {
			return "", err
		}
		appDir = filepath.Join("data", "data", string(bytes.Trim(data, "\x00")))
		return appDir, nil
	}

	home, err := os.UserHomeDir()
	if err != nil {
		return "", err
	}
	appDir = filepath.Join(home, ".bloom")
	return appDir, nil
}

package kernel

import (
	"bytes"
	"io/ioutil"
	"os"
	"path/filepath"
	"runtime"
)

// userDataDir is used to cache UserDataDir
var userDataDir string

// UserDataDir returns the directory which should be used to store user data
func UserDataDir() (string, error) {
	if userDataDir != "" {
		return userDataDir, nil
	}

	goos := runtime.GOOS

	if goos == "android" {
		data, err := ioutil.ReadFile("/proc/self/cmdline")
		if err != nil {
			return "", err
		}
		userDataDir = filepath.Join("data", "data", string(bytes.Trim(data, "\x00")))
		return userDataDir, nil
	} else {
		configDir, err := os.UserConfigDir()
		if err != nil {
			return "", err
		}
		userDataDir = filepath.Join(configDir, AppID())
		return userDataDir, nil
	}
}

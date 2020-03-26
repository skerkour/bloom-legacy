package kernel

import (
	"bytes"
	"io/ioutil"
	"path/filepath"
	"runtime"

	"gitlab.com/bloom42/lily/appdir"
)

// appDataDir is used to cache AppDataDir
var appDataDir string

// AppDataDir returns the directory which should be used to store user data
func AppDataDir() (string, error) {
	var err error
	if appDataDir != "" {
		return appDataDir, nil
	}

	goos := runtime.GOOS

	if goos == "android" {
		data, err := ioutil.ReadFile("/proc/self/cmdline")
		if err != nil {
			return "", err
		}
		appDataDir = filepath.Join("data", "data", string(bytes.Trim(data, "\x00")))
		return appDataDir, nil
	}

	dirs := appdir.New(AppID())
	appDataDir, err = dirs.UserData()
	return appDataDir, err
}

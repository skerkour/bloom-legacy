package kernel

import (
	"bytes"
	"io/ioutil"
	"os"
	"path/filepath"
	"runtime"
	"strings"
)

// appDataDir is used to cache AppDataDir
var appDataDir string

// AppDataDir returns the directory which should be used to store user data
func AppDataDir() (string, error) {
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
	} else {
		configDir, err := os.UserConfigDir()
		if err != nil {
			return "", err
		}
		appId := AppID()
		// because sometimes, like with flatpak, UserConfigDir can already be app scoped
		if strings.Contains(configDir, appId) {
			appDataDir = configDir
		} else {
			appDataDir = filepath.Join(configDir, appId)
		}
		return appDataDir, nil
	}
}

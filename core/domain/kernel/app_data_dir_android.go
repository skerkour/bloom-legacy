package kernel

import (
	"bytes"
	"io/ioutil"
	"path/filepath"
)

// appDataDir is used to cache AppDataDir
var appDataDir string

// AppDataDir returns the directory which should be used to store user data
func AppDataDir() (string, error) {
	var err error
	if appDataDir != "" {
		return appDataDir, nil
	}

	data, err := ioutil.ReadFile("/proc/self/cmdline")
	if err != nil {
		return "", err
	}
	appDataDir = filepath.Join("data", "data", string(bytes.Trim(data, "\x00")))
	return appDataDir, nil

}

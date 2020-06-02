package kernel

import (
	"gitlab.com/bloom42/gobox/appdir"
)

// appDataDir is used to cache AppDataDir
var appDataDir string

// AppDataDir returns the directory which should be used to store user data
func AppDataDir() (string, error) {
	var err error
	if appDataDir != "" {
		return appDataDir, nil
	}

	dirs := appdir.New(AppID)
	appDataDir, err = dirs.UserData()
	return appDataDir, err
}

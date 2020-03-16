package kernel

import (
	"bytes"
	"io/ioutil"
	"os/user"
	"path/filepath"
	"runtime"
)

func AppDirectory() (string, error) {
	if runtime.GOOS == "android" {
		data, err := ioutil.ReadFile("/proc/self/cmdline")
		if err != nil {
			return "", err
		}
		return filepath.Join("data", "data", string(bytes.Trim(data, "\x00"))), nil
	} else {
		home, err := homeDirectory()
		if err != nil {
			return "", err
		}
		return filepath.Join(home, ".bloom"), nil
	}
}

func homeDirectory() (string, error) {
	user, err := user.Current()
	if err != nil {
		return "", err
	}
	return user.HomeDir, nil
}

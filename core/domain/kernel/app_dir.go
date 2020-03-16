package kernel

import (
	"bytes"
	"io/ioutil"
	"os"
	"path/filepath"
	"runtime"
)

func AppDir() (string, error) {
	if runtime.GOOS == "android" {
		data, err := ioutil.ReadFile("/proc/self/cmdline")
		if err != nil {
			return "", err
		}
		return filepath.Join("data", "data", string(bytes.Trim(data, "\x00"))), nil
	} else {
		home, err := os.UserHomeDir()
		if err != nil {
			return "", err
		}
		return filepath.Join(home, ".bloom"), nil
	}
}

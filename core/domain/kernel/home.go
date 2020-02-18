package kernel

import (
	"os/user"
)

func GetHomeDirectory() (string, error) {
	user, err := user.Current()
	if err != nil {
		return "", err
	}
	return user.HomeDir, nil
}

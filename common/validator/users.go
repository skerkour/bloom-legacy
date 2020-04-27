package validator

import (
	"errors"
	"fmt"
)

const (
	PASSWORD_MIN_LENGTH = 8
	PASSWORD_MAX_LENGTH = 2048
)

// Password validates a password
func Password(password string, basicPassword map[string]bool) error {
	passwordLength := len(password)

	if passwordLength < PASSWORD_MIN_LENGTH {
		return fmt.Errorf("Password must be longer than %d characters", PASSWORD_MIN_LENGTH-1)
	}

	if passwordLength > PASSWORD_MAX_LENGTH {
		return fmt.Errorf("Password must be shorter than %d characters", PASSWORD_MAX_LENGTH)
	}

	if _, ok := basicPassword[password]; ok {
		return errors.New("Password is too weak")
	}

	return nil
}

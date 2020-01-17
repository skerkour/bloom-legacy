package api

import (
	"net"

	"errors"
)

func ValidateIP(ip string) error {
	parsedIP := net.ParseIP(ip)
	if parsedIP == nil {
		return errors.New("ip is not valid")
	}

	return nil
}

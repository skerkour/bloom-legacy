package api

import (
	"net"

	"errors"
)

// ValidateIP verfies that a given IP address is valid
func ValidateIP(ip string) error {
	parsedIP := net.ParseIP(ip)
	if parsedIP == nil {
		return errors.New("IP is not valid")
	}

	return nil
}

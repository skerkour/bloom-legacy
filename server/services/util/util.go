package util

import (
	"crypto/rand"
	"math/big"
	mathrand "math/rand"
)

const (
	DigitAlphabet = "0123456789"
)

func Init() error {
	seed, err := rand.Int(rand.Reader, big.NewInt(9223372036854775807)) // which is mac int64
	if err != nil {
		return err
	}
	mathrand.Seed(seed.Int64())
	return nil
}

func InsecureRandomInt(min, max int) int {
	return mathrand.Intn(max-min) + min
}

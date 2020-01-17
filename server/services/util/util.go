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

func RandomInt64(min, max int64) (int64, error) {
	n, err := rand.Int(rand.Reader, big.NewInt(max-min))
	if err != nil {
		return max, err
	}
	return n.Int64(), nil
}

func RandomDigitStr(n uint) (string, error) {
	b := make([]byte, n)

	for i := range b {
		n, err := rand.Int(rand.Reader, big.NewInt(int64(len(DigitAlphabet))))
		if err != nil {
			return "", err
		}
		b[i] = DigitAlphabet[n.Int64()]
	}
	return string(b), nil
}

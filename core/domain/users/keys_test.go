package users

import (
	"bytes"
	"testing"

	"gitlab.com/bloom42/lily/crypto"
)

func TestDerivePasswordKeyFromPassword(t *testing.T) {
	password := []byte("password")
	username := []byte("username")
	expectedKeySize := 64
	zeroBuffer := make([]byte, expectedKeySize)

	key, err := derivePasswordKeyFromPassword(password, username)
	if err != nil {
		t.Error(err)
	}

	if len(key) != expectedKeySize {
		t.Errorf("Invalid key size (%d while %d is expected)", len(key), expectedKeySize)
	}

	if bytes.Equal(key, zeroBuffer) {
		t.Error("Key is empty")
	}
}

func TestDeriveWrapKeyFromPasswordKey(t *testing.T) {
	username := []byte("username")
	expectedKeySize := 32
	zeroBuffer := make([]byte, expectedKeySize)
	passwordKey, err := crypto.RandBytes(crypto.KeySize512)
	if err != nil {
		t.Error(err)
	}

	key, err := deriveWrapKeyFromPasswordKey(passwordKey, username)
	if err != nil {
		t.Error(err)
	}

	if len(key) != expectedKeySize {
		t.Errorf("Invalid key size (%d while %d is expected)", len(key), expectedKeySize)
	}

	if bytes.Equal(key, zeroBuffer) {
		t.Error("Key is empty")
	}
}

func TestDeriveAuthKeyFromPasswordKey(t *testing.T) {
	username := []byte("username")
	expectedKeySize := 64
	zeroBuffer := make([]byte, expectedKeySize)
	passwordKey, err := crypto.RandBytes(crypto.KeySize512)
	if err != nil {
		t.Error(err)
	}

	key, err := deriveAuthKeyFromPasswordKey(passwordKey, username)
	if err != nil {
		t.Error(err)
	}

	if len(key) != expectedKeySize {
		t.Errorf("Invalid key size (%d while %d is expected)", len(key), expectedKeySize)
	}

	if bytes.Equal(key, zeroBuffer) {
		t.Error("Key is empty")
	}
}

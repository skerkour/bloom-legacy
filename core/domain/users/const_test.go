package users

import (
	"testing"
)

func TestPreferencesKeys(t *testing.T) {
	keys := []string{
		MASTER_KEY_KEY,
		PRIVATE_KEY_KEY,
		PUBLIC_KEY_KEY,
		ME_KEY,
		SESSION_KEY,
	}
	expected := []string{
		"master_key",
		"private_key",
		"public_key",
		"me",
		"session",
	}

	for i, key := range keys {
		if key != expected[i] {
			t.Errorf("%s is invalid (%s expected)", key, expected[i])
		}
	}
}

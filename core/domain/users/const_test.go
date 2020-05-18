package users

import (
	"testing"
)

func TestPreferencesKeys(t *testing.T) {
	keys := []string{
		PREFERENCES_KEY_MASTER_KEY,
		PREFERENCES_KEY_PRIVATE_KEY,
		PREFERENCES_KEY_PUBLIC_KEY,
		PREFERENCES_KEY_ME,
		PREFERENCES_KEY_SESSION,
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

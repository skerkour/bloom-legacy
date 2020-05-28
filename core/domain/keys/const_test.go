package keys

import (
	"testing"
)

func TestPreferencesKeys(t *testing.T) {
	keys := []string{
		PREFERENCES_KEY_MASTER_KEY,
		PREFERENCES_KEY_PRIVATE_KEY,
		PREFERENCES_KEY_PUBLIC_KEY,
	}
	expected := []string{
		"master_key",
		"private_key",
		"public_key",
	}

	for i, key := range keys {
		if key != expected[i] {
			t.Errorf("%s is invalid (%s expected)", key, expected[i])
		}
	}
}

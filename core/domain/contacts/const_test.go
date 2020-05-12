package contacts

import (
	"testing"
)

func TestContactType(t *testing.T) {
	if CONTACT_TYPE != "com.bloom42.bloom.contact" {
		t.Errorf("CONTACT_TYPE is invalid")
	}
}

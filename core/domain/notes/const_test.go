package notes

import (
	"testing"
)

func TestNoteType(t *testing.T) {
	if NOTE_TYPE != "com.bloom42.bloom.note" {
		t.Errorf("NOTE_TYPE is invalid")
	}
}

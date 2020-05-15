package kernel

import (
	"testing"
)

func TestObjectTypes(t *testing.T) {
	values := []string{
		OBJECT_TYPE_NOTE,
		OBJECT_TYPE_CONTACT,
		OBJECT_TYPE_CALENDAR_EVENT,
	}
	expected := []string{
		"com.bloom42.bloom.note",
		"com.bloom42.contacts.contact",
		"com.bloom42.calendar.event",
	}

	for i, val := range values {
		if val != expected[i] {
			t.Errorf("Invalid object type: %s while %s is expected", val, expected[i])
		}
	}
}

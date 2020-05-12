package calendar

import (
	"testing"
)

func TestEventType(t *testing.T) {
	if EVENT_TYPE != "com.bloom42.bloom.calendar_event" {
		t.Errorf("EVENT_TYPE is invalid")
	}
}

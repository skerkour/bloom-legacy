package repository

import (
	"fmt"
	"testing"

	"gitlab.com/bloom42/gobox/uuid"
)

func TestGetSessionCacheKey(t *testing.T) {
	id := uuid.New()
	expected := fmt.Sprintf("sessions/%s", id.String())

	got := getSessionCacheKey(id)

	if expected != got {
		t.Errorf("expected: %s; got: %s", expected, got)
	}
}

func TestGetUserCacheKey(t *testing.T) {
	id := uuid.New()
	expected := fmt.Sprintf("users/%s", id.String())

	got := getUserCacheKey(id)

	if expected != got {
		t.Errorf("expected: %s; got: %s", expected, got)
	}
}

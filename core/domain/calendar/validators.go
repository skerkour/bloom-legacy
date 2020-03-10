package calendar

import (
	"errors"
	"time"
)

func valdiateEventDates(startAt, endAt time.Time) error {
	if endAt.Before(startAt) {
		return errors.New("endAt can't be before startAt")
	}
	return nil
}

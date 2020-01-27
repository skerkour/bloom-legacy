package calendar

import (
	"errors"
	"time"
)

func valdiateEventDates(startAt, endAt time.Time) error {
	if endAt.Before(startAt) {
		return errors.New("end_at can't be before start_at")
	}
	return nil
}

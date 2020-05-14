package calendar

import (
	"context"

	"gitlab.com/bloom42/bloom/core/domain/objects"
	"gitlab.com/bloom42/bloom/core/messages"
)

func DeleteEvent(params messages.CalendarDeleteEventParams) error {
	err := objects.DeleteObject(context.Background(), nil, params.ID)

	// request sync
	objects.SyncChan <- true

	return err
}

package calendar

import (
	"gitlab.com/bloom42/bloom/core/db"
	"gitlab.com/bloom42/bloom/core/domain/kernel"
)

func DeleteEvent(params DeleteEventParams) (kernel.Empty, error) {
	ret := kernel.Empty{}

	query := "DELETE FROM calendar_events WHERE id = ?"
	_, err := db.DB.Exec(query, params.ID)

	return ret, err
}

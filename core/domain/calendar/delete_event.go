package calendar

import (
	"gitlab.com/bloom42/bloom/core/db"
	"gitlab.com/bloom42/bloom/core/messages"
)

func DeleteEvent(params DeleteEventParams) (messages.Empty, error) {
	ret := messages.Empty{}

	query := "DELETE FROM calendar_events WHERE id = ?"
	_, err := db.DB.Exec(query, params.ID)

	return ret, err
}

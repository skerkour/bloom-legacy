package notes

import (
	"gitlab.com/bloom42/bloom/core/db"
	"gitlab.com/bloom42/bloom/core/messages"
)

func DeleteNote(params messages.DeleteNoteParams) (messages.Empty, error) {
	ret := messages.Empty{}

	stmt, err := db.DB.Prepare("DELETE FROM notes WHERE id = ?")
	if err != nil {
		return ret, err
	}
	defer stmt.Close()

	_, err = stmt.Exec(&params.ID)

	return ret, err
}

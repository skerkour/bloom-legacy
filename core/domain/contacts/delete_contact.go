package contacts

import (
	"gitlab.com/bloom42/bloom/core/db"
)

func DeleteContact(params DeleteContactParams) error {
	_, err := db.DB.Exec("DELETE FROM contacts WHERE id = ?", params.ID)
	return err
}

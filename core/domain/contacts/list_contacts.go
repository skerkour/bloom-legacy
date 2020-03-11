package contacts

import (
	"gitlab.com/bloom42/bloom/core/db"
)

func ListContacts() (Contacts, error) {
	var err error
	ret := Contacts{Contacts: []Contact{}}

	query := `SELECT id, created_at, updated_at, first_name, last_name, notes, addresses,
		birthday, organizations, emails, phones, websites, device_id
		FROM contacts`
	err = db.DB.Select(&ret.Contacts, query)

	return ret, err
}

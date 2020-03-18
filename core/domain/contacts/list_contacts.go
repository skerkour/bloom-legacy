package contacts

import (
	"gitlab.com/bloom42/bloom/core/db"
)

func ListContacts() (Contacts, error) {
	var err error
	ret := Contacts{Contacts: []Contact{}}

	query := `SELECT * FROM contacts`
	err = db.DB.Select(&ret.Contacts, query)

	return ret, err
}

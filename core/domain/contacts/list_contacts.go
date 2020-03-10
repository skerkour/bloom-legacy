package contacts

import (
	"gitlab.com/bloom42/bloom/core/db"
)

func ListContacts() (Contacts, error) {
	ret := Contacts{Contacts: []Contact{}}

	rows, err := db.DB.Query(`SELECT id, created_at, updated_at, first_name, last_name, notes, addresses,
		birthday, organizations, emails, phones, websites, device_id
		FROM contacts`)
	if err != nil {
		return ret, err
	}
	defer rows.Close()

	for rows.Next() {
		var contact Contact
		err := rows.Scan(&contact.ID, &contact.CreatedAt, &contact.UpdatedAt, &contact.FirstName, &contact.LastName,
			&contact.Notes, &contact.Addresses, &contact.Birthday, &contact.Organizations, &contact.Emails, &contact.Phones,
			&contact.Websites, &contact.DeviceID)
		if err != nil {
			return ret, err
		}
		ret.Contacts = append(ret.Contacts, contact)
	}

	return ret, nil
}

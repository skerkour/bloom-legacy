package contacts

import (
	"strings"
	"time"

	"gitlab.com/bloom42/bloom/core/db"
)

func UpdateContact(contact Contact) (Contact, error) {
	// TODO: validators
	if len(contact.Emails) == 1 && contact.Emails[0].Email == "" &&
		(contact.Emails[0].Label == "" || strings.ToLower(contact.Emails[0].Label) == "other") {
		contact.Emails = Emails{}
	}
	if len(contact.Websites) == 1 && contact.Websites[0].Website == "" &&
		(contact.Websites[0].Label == "" || strings.ToLower(contact.Websites[0].Label) == "other") {
		contact.Websites = Websites{}
	}
	if len(contact.Phones) == 1 && contact.Phones[0].Phone == "" &&
		(contact.Phones[0].Label == "" || strings.ToLower(contact.Phones[0].Label) == "other") {
		contact.Phones = Phones{}
	}
	if len(contact.Organizations) == 1 && contact.Organizations[0].Name == "" &&
		(contact.Organizations[0].Title == "" || strings.ToLower(contact.Organizations[0].Title) == "other") {
		contact.Organizations = Organizations{}
	}

	var err error
	now := time.Now().UTC()

	contact.UpdatedAt = now

	query := `
		UPDATE contacts SET
			updated_at = $1,
			first_name = $2,
			last_name = $3,
			notes = $4,
			addresses = $5,
			birthday = $6,
			organizations = $7,
			emails = $8,
			phones = $9,
			websites = $10,
			device_id = $11
		WHERE id = $12
	`
	_, err = db.DB.Exec(query,
		&contact.UpdatedAt,
		&contact.FirstName,
		&contact.LastName,
		&contact.Notes,
		&contact.Addresses,
		&contact.Birthday,
		&contact.Organizations,
		&contact.Emails,
		&contact.Phones,
		&contact.Websites,
		&contact.DeviceID,
		&contact.ID,
	)

	return contact, err
}

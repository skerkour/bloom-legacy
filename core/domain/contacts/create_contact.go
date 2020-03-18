package contacts

import (
	"fmt"
	"strings"
	"time"

	"github.com/google/uuid"
	"gitlab.com/bloom42/bloom/core/db"
)

func CreateContact(params CreateContactParams) (Contact, error) {
	// TODO(z0mbie42): validators
	if len(params.Emails) == 1 && params.Emails[0].Email == "" &&
		(params.Emails[0].Label == "" || strings.ToLower(params.Emails[0].Label) == "other") {
		params.Emails = Emails{}
	}
	if len(params.Websites) == 1 && params.Websites[0].Website == "" &&
		(params.Websites[0].Label == "" || strings.ToLower(params.Websites[0].Label) == "other") {
		params.Websites = Websites{}
	}
	if len(params.Phones) == 1 && params.Phones[0].Phone == "" &&
		(params.Phones[0].Label == "" || strings.ToLower(params.Phones[0].Label) == "other") {
		params.Phones = Phones{}
	}
	if len(params.Organizations) == 1 && params.Organizations[0].Name == "" &&
		(params.Organizations[0].Title == "" || strings.ToLower(params.Organizations[0].Title) == "other") {
		params.Organizations = Organizations{}
	}

	var err error
	now := time.Now().UTC()
	uuid := uuid.New()
	contact := Contact{
		ID:            uuid.String(),
		CreatedAt:     now,
		UpdatedAt:     now,
		DeviceID:      params.DeviceID,
		FirstName:     params.FirstName,
		LastName:      params.LastName,
		Notes:         params.Notes,
		Birthday:      params.Birthday,
		Organizations: params.Organizations,
		Addresses:     params.Addresses,
		Emails:        params.Emails,
		Phones:        params.Phones,
		Websites:      params.Websites,
	}

	fmt.Printf("Contact: %#v\n", contact)

	query := `
	INSERT INTO contacts
		(id,
		created_at,
		updated_at,
		first_name,
		last_name,
		notes,
		addresses,
		birthday,
		organizations,
		emails,
		phones,
		websites,
		device_id,
		bloom_username)
	VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
	`
	_, err = db.DB.Exec(query,
		&contact.ID,
		&contact.CreatedAt,
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
		contact.BloomUsername,
	)

	return contact, err
}

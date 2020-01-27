package contacts

import (
	"time"

	"github.com/google/uuid"
	"gitlab.com/bloom42/bloom/core/db"
)

func CreateContact(params CreateContactParams) (Contact, error) {
	// TODO(z0mbie42): validators
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

	stmt, err := db.DB.Prepare(`
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
		device_id)
	VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
	`)
	if err != nil {
		return contact, err
	}
	defer stmt.Close()

	_, err = stmt.Exec(&contact.ID,
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
	)

	return contact, err
}

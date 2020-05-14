package contacts

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/core/domain/kernel"
	"gitlab.com/bloom42/bloom/core/domain/objects"
)

func CreateContact(params CreateContactParams) (*objects.Object, error) {
	// TODO(z0mbie42): validators
	var err error
	var ret *objects.Object

	now := time.Now().UTC()
	contact := Contact{
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

	cleanContactCollections(&contact)

	id, err := objects.GenerateObjectID([]byte(kernel.Me.Username))
	if err != nil {
		return ret, err
	}

	ret, err = objects.ToObject(id, CONTACT_TYPE, now, now, nil, true, &contact)
	if err != nil {
		return ret, err
	}

	err = objects.SaveObject(context.Background(), nil, ret)

	// request sync
	objects.SyncChan <- true

	return ret, err
}

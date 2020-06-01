package contacts

import (
	"context"

	"gitlab.com/bloom42/bloom/core/domain/kernel"
	"gitlab.com/bloom42/bloom/core/domain/objects"
)

func ListContacts() (Contacts, error) {
	ret := Contacts{Contacts: []objects.Object{}}

	objects, err := objects.FindObjectsByType(context.Background(), nil, kernel.OBJECT_TYPE_CONTACT, nil)
	if err != nil {
		return ret, err
	}

	ret.Contacts = objects

	return ret, nil
}

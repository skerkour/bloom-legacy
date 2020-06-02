package contacts

import (
	"encoding/json"
	"time"

	"gitlab.com/bloom42/bloom/core/domain/objects"
)

type Contacts struct {
	Contacts []objects.Object `json:"contacts"`
}

type Contact struct {
	IsFavorite    bool                `json:"isFavorite"`
	DeviceID      string              `json:"deviceId"`
	Prefix        string              `json:"prefix"`
	Suffix        string              `json:"suffix"`
	Nickname      string              `json:"nickname"`
	FirstName     string              `json:"firstName"`
	LastName      string              `json:"lastName"`
	BloomUsername string              `json:"bloomUsername"`
	Notes         string              `json:"notes"`
	PublicKey     []byte              `json:"publicKey"`
	Birthday      *time.Time          `json:"birthday"`
	Organizations Organizations       `json:"organizations"`
	Addresses     Addresses           `json:"addresses"`
	Emails        ContactInformations `json:"emails"`
	Phones        ContactInformations `json:"phones"`
	Online        ContactInformations `json:"websites"`
}

func ObjectToContact(object *objects.Object) (ret *Contact, err error) {
	ret = &Contact{}
	err = json.Unmarshal(object.Data, ret)
	return
}

type Organization struct {
	Name  string `json:"name"`
	Title string `json:"title"`
}

type Organizations []Organization

type Address struct {
	Street     string `json:"street"`
	City       string `json:"city"`
	State      string `json:"state"`
	PostalCode string `json:"postalCode"`
	Country    string `json:"country"`
	Label      string `json:"label"`
}

type Addresses []Address

type ContactInformation struct {
	Value string `json:"value"`
	Label string `json:"label"`
}

type ContactInformations []ContactInformation

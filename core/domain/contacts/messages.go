package contacts

import (
	"time"
)

type CreateContactParams struct {
	DeviceID      string              `json:"deviceId"`
	Prefix        string              `json:"prefix"`
	Suffix        string              `json:"suffix"`
	Nickname      string              `json:"nickname"`
	FirstName     string              `json:"firstName"`
	LastName      string              `json:"lastName"`
	Notes         string              `json:"notes"`
	Birthday      *time.Time          `json:"birthday"`
	BloomUsername string              `json:"bloomUsername" db:"bloom_username"`
	Organizations Organizations       `json:"organizations"`
	Addresses     Addresses           `json:"addresses"`
	Emails        ContactInformations `json:"emails"`
	Phones        ContactInformations `json:"phones"`
	Online        ContactInformations `json:"online"`
}

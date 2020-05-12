package contacts

import (
	"database/sql/driver"
	"encoding/json"
	"errors"
	"time"

	"gitlab.com/bloom42/bloom/core/domain/objects"
)

type Contacts struct {
	Contacts []objects.Object `json:"contacts"`
}

type Contact struct {
	DeviceID      string        `json:"deviceId" db:"device_id"`
	FirstName     string        `json:"firstName" db:"first_name"`
	LastName      string        `json:"lastName" db:"last_name"`
	BloomUsername string        `json:"bloomUsername" db:"bloom_username"`
	Notes         string        `json:"notes" db:"notes"`
	Birthday      *time.Time    `json:"birthday" db:"birthday"`
	Organizations Organizations `json:"organizations" db:"organizations"`
	Addresses     Addresses     `json:"addresses" db:"addresses"`
	Emails        Emails        `json:"emails" db:"emails"`
	Phones        Phones        `json:"phones" db:"phones"`
	Websites      Websites      `json:"websites" db:"websites"`
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

func (o Organizations) Value() (driver.Value, error) {
	data, err := json.Marshal(o)
	if err != nil {
		return nil, err
	}
	return string(data), nil
}

func (o *Organizations) Scan(src interface{}) error {
	sourceStr, ok := src.(string)
	if !ok {
		return errors.New("Incompatible type for contacts.Organizations")
	}
	err := json.Unmarshal([]byte(sourceStr), o)
	return err
}

type Address struct {
	Street     string `json:"street"`
	City       string `json:"city"`
	State      string `json:"state"`
	PostalCode string `json:"postalCode"`
	Country    string `json:"country"`
	Label      string `json:"label"`
}

type Addresses []Address

func (a Addresses) Value() (driver.Value, error) {
	data, err := json.Marshal(a)
	if err != nil {
		return nil, err
	}
	return string(data), nil
}

func (a *Addresses) Scan(src interface{}) error {
	sourceStr, ok := src.(string)
	if !ok {
		return errors.New("Incompatible type for contacts.Addresses")
	}
	err := json.Unmarshal([]byte(sourceStr), a)
	return err
}

type Email struct {
	Email string `json:"email"`
	Label string `json:"label"`
}

type Emails []Email

func (e Emails) Value() (driver.Value, error) {
	data, err := json.Marshal(e)
	if err != nil {
		return nil, err
	}
	return string(data), nil
}

func (e *Emails) Scan(src interface{}) error {
	sourceStr, ok := src.(string)
	if !ok {
		return errors.New("Incompatible type for contacts.Emails")
	}
	err := json.Unmarshal([]byte(sourceStr), e)
	return err
}

type Phone struct {
	Phone string `json:"phone"`
	Label string `json:"label"`
}

type Phones []Phone

func (p Phones) Value() (driver.Value, error) {
	data, err := json.Marshal(p)
	if err != nil {
		return nil, err
	}
	return string(data), nil
}

func (p *Phones) Scan(src interface{}) error {
	sourceStr, ok := src.(string)
	if !ok {
		return errors.New("Incompatible type for contacts.Phones")
	}
	err := json.Unmarshal([]byte(sourceStr), p)
	return err
}

type Website struct {
	Website string `json:"website"`
	Label   string `json:"label"`
}

type Websites []Website

func (w Websites) Value() (driver.Value, error) {
	data, err := json.Marshal(w)
	if err != nil {
		return nil, err
	}
	return string(data), nil
}

func (w *Websites) Scan(src interface{}) error {
	sourceStr, ok := src.(string)
	if !ok {
		return errors.New("Incompatible type for contacts.Websites")
	}
	err := json.Unmarshal([]byte(sourceStr), w)
	return err
}

package contacts

import (
	"database/sql/driver"
	"encoding/json"
	"errors"
	"time"
)

type Contact struct {
	ID            string         `json:"id"`
	CreatedAt     time.Time      `json:"created_at"`
	UpdatedAt     time.Time      `json:"updated_at"`
	DeviceId      string         `json:"device_id"`
	FirstName     string         `json:"first_name"`
	LastName      string         `json:"last_name"`
	Notes         string         `json:"notes"`
	Birthday      *time.Time     `json:"birthday"`
	Organizations []Organization `json:"organizations"`
	Addresses     []Address      `json:"addresses"`
	Emails        []Email        `json:"emails"`
	Phones        []Phone        `json:"phones"`
	Websites      []Website      `json:"websites"`
}

type Organization struct {
	Name  string `json:"name"`
	Title string `json:"title"`
}

func (o Organization) Value() (driver.Value, error) {
	data, err := json.Marshal(o)
	if err != nil {
		return nil, err
	}
	return string(data), nil
}

func (o *Organization) Scan(src interface{}) error {
	sourceStr, ok := src.(string)
	if !ok {
		return errors.New("Incompatible type for contacts.Organization")
	}
	err := json.Unmarshal([]byte(sourceStr), o)
	return err
}

type Address struct {
	City           string `json:"city"`
	Country        string `json:"country"`
	Label          string `json:"label"`
	PostalCode     string `json:"postal_code"`
	StreetAddress  string `json:"street_address"`
	StreetAddress2 string `json:"street_address2"`
}

func (a Address) Value() (driver.Value, error) {
	data, err := json.Marshal(a)
	if err != nil {
		return nil, err
	}
	return string(data), nil
}

func (a *Address) Scan(src interface{}) error {
	sourceStr, ok := src.(string)
	if !ok {
		return errors.New("Incompatible type for contacts.Address")
	}
	err := json.Unmarshal([]byte(sourceStr), a)
	return err
}

type Email struct {
	Email string `json:"email"`
	Label string `json:"label"`
}

func (e Email) Value() (driver.Value, error) {
	data, err := json.Marshal(e)
	if err != nil {
		return nil, err
	}
	return string(data), nil
}

func (e *Email) Scan(src interface{}) error {
	sourceStr, ok := src.(string)
	if !ok {
		return errors.New("Incompatible type for contacts.Email")
	}
	err := json.Unmarshal([]byte(sourceStr), e)
	return err
}

type Phone struct {
	Phone string `json:"phone"`
	Label string `json:"label"`
}

func (p Phone) Value() (driver.Value, error) {
	data, err := json.Marshal(p)
	if err != nil {
		return nil, err
	}
	return string(data), nil
}

func (p *Phone) Scan(src interface{}) error {
	sourceStr, ok := src.(string)
	if !ok {
		return errors.New("Incompatible type for contacts.Phone")
	}
	err := json.Unmarshal([]byte(sourceStr), p)
	return err
}

type Website struct {
	Website string `json:"website"`
	Label   string `json:"label"`
}

func (w Website) Value() (driver.Value, error) {
	data, err := json.Marshal(w)
	if err != nil {
		return nil, err
	}
	return string(data), nil
}

func (w *Website) Scan(src interface{}) error {
	sourceStr, ok := src.(string)
	if !ok {
		return errors.New("Incompatible type for contacts.Website")
	}
	err := json.Unmarshal([]byte(sourceStr), w)
	return err
}

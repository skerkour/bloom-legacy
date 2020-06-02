package contacts

import "strings"

func isEmailEmpty(email ContactInformation) bool {
	label := strings.ToLower(email.Label)
	if email.Value == "" && (label == "" || label == "other" || label == "personal") {
		return true
	}
	return false
}

func isWebsiteEmpty(website ContactInformation) bool {
	label := strings.ToLower(website.Label)
	if website.Value == "" && (label == "" || label == "other" || label == "personal") {
		return true
	}
	return false
}

func isPhoneEmpty(phone ContactInformation) bool {
	label := strings.ToLower(phone.Label)
	if phone.Value == "" && (label == "" || label == "other" || label == "mobile") {
		return true
	}
	return false
}

func isOrganizationEmpty(org Organization) bool {
	if org.Name == "" && org.Title == "" {
		return true
	}
	return false
}

func isAddressEmpty(address Address) bool {
	label := strings.ToLower(address.Label)
	if address.Street == "" && address.City == "" && address.Country == "" &&
		address.PostalCode == "" && address.State == "" &&
		(label == "" || label == "other" || label == "home") {
		return true
	}
	return false
}

func cleanContactCollections(contact *Contact) {
	n := 0

	for _, email := range contact.Emails {
		if !isEmailEmpty(email) {
			contact.Emails[n] = email
			n++
		}
	}
	contact.Emails = contact.Emails[:n]

	n = 0
	for _, website := range contact.Online {
		if !isWebsiteEmpty(website) {
			contact.Online[n] = website
			n++
		}
	}
	contact.Online = contact.Online[:n]

	n = 0
	for _, phone := range contact.Phones {
		if !isPhoneEmpty(phone) {
			contact.Phones[n] = phone
			n++
		}
	}
	contact.Phones = contact.Phones[:n]

	n = 0
	for _, org := range contact.Organizations {
		if !isOrganizationEmpty(org) {
			contact.Organizations[n] = org
			n++
		}
	}
	contact.Organizations = contact.Organizations[:n]

	n = 0
	for _, address := range contact.Addresses {
		if !isAddressEmpty(address) {
			contact.Addresses[n] = address
			n++
		}
	}
	contact.Addresses = contact.Addresses[:n]

}

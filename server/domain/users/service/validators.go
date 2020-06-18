package service

import (
	"fmt"
	"regexp"
	"strings"

	"github.com/asaskevich/govalidator"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/errors"
)

func validateFirstName(firstName string) error {
	firstNameLen := len(firstName)

	if firstNameLen == 0 {
		return errors.InvalidArgument("first_name cannot be empty")
	}

	if firstNameLen > users.FirstNameMaxLength {
		return errors.InvalidArgument("first_name is too long")
	}

	return nil
}

func validateLastName(lastName string) error {
	lastNameLen := len(lastName)

	if lastNameLen == 0 {
		return errors.InvalidArgument("last_name cannot be empty")
	}

	if lastNameLen > users.LastNameMaxLength {
		return errors.InvalidArgument("last_name is too long")
	}

	return nil
}

func validateBio(bio string) error {
	if len(bio) > users.BioMaxLength {
		return errors.InvalidArgument("bio is too long")
	}

	return nil
}

func validateDisplayName(displayName string) error {
	displayNameLen := len(displayName)

	if displayNameLen == 0 {
		return errors.InvalidArgument("display_name cannot be empty")
	}

	if displayNameLen > users.DisplayNameMaxLength {
		return errors.InvalidArgument("display_name is too long")
	}

	return nil
}

// TODO
/*
pub fn email<S: std::hash::BuildHasher>(
    disposable_emails: HashSet<String, S>,
    email: &str,
) -> Result<(), BloomError> {
    if email.is_empty() || !email.contains('@') {
        return Err(BloomError::Validation("email is not valid".to_string()));
    }

    let parts: Vec<&str> = email.split('@').collect();

    if parts.len() != 2 {
        return Err(BloomError::Validation("email is not valid".to_string()));
    }

    let user_part = parts[1];
    let domain_part = parts[0];

    if user_part.is_empty() || domain_part.is_empty() {
        return Err(BloomError::Validation("email is not valid".to_string()));
    }

    if email.trim() != email {
        return Err(BloomError::Validation(
            "email must not contains whitesapces".to_string(),
        ));
    }

    if email.len() > 128 {
        return Err(BloomError::Validation("email is too long".to_string()));
    }

    let user_re = Regex::new(r"^(?i)[a-z0-9.!#$%&'*+/=?^_`{|}~-]+\z")
        .expect("error compiling user email regex");
    let domain_re = Regex::new(
        r"(?i)^[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)*$",
    )
    .expect("error compiling domain email regex");

    if !user_re.is_match(user_part) {
        return Err(BloomError::Validation("email is not valid".to_string()));
    }
    if !domain_re.is_match(domain_part) {
        return Err(BloomError::Validation("email is not valid".to_string()));
    }

    if disposable_emails.contains(&domain_part.to_string()) {
        return Err(BloomError::Validation(
            "email domain is not valid".to_string(),
        ));
    }

    return Ok(());
}
*/

func validateEmail(email string, disposableEmailDomains map[string]bool) error {
	if !govalidator.IsEmail(email) {
		return errors.InvalidArgument("email is not valid")
	}
	return nil
}

var isAlphaNumeric = regexp.MustCompile(`^[a-z0-9]+$`).MatchString

func validateUsername(username string) error {
	usernameLength := len(username)

	if usernameLength == 0 {
		return errors.InvalidArgument("username cannot be empty")
	}

	if usernameLength < users.UsernameMinLength {
		return errors.InvalidArgument(fmt.Sprintf("username must be longer than %d characters", users.UsernameMinLength-1))
	}

	if usernameLength > users.UsernameMaxLength {
		return errors.InvalidArgument(fmt.Sprintf("username must be longer than %d characters", users.UsernameMaxLength))
	}

	if username != strings.ToLower(username) {
		return errors.InvalidArgument("username must be lowercase")
	}

	if !isAlphaNumeric(username) {
		return errors.InvalidArgument("username must contains only alphanumeric characters")
	}

	if strings.Contains(username, "admin") ||
		strings.Contains(username, "bloom") ||
		strings.HasSuffix(username, "bot") ||
		stringSliceContains(users.InvalidUsername, username) {
		return errors.InvalidArgument("username is not valid")
	}

	return nil
}

func stringSliceContains(s []string, e string) bool {
	for _, a := range s {
		if a == e {
			return true
		}
	}
	return false
}

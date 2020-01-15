package accounts

import (
	"errors"
	"gitlab.com/bloom42/bloom/core/consts"
	"regexp"
	"strings"
)

func validateFirstName(firstName string) error {
	firstNameLen := len(firstName)

	if firstNameLen == 0 {
		return errors.New("first_name cannot be empty")
	}

	if firstNameLen > consts.FIRST_NAME_MAX_LENGTH {
		return errors.New("first_name is too long")
	}

	return nil
}

func validateLastName(lastName string) error {
	lastNameLen := len(lastName)

	if lastNameLen == 0 {
		return errors.New("last_name cannot be empty")
	}

	if lastNameLen > consts.LAST_NAME_MAX_LENGTH {
		return errors.New("last_name is too long")
	}

	return nil
}

func validateBio(bio string) error {
	if len(bio) > consts.BIO_MAX_LENGTH {
		return errors.New("bio is too long")
	}

	return nil
}

func validateDisplayName(displayName string) error {
	displayNameLen := len(displayName)

	if displayNameLen == 0 {
		return errors.New("display_name cannot be empty")
	}

	if displayNameLen > consts.DISPLAY_NAME_MAX_LENGTH {
		return errors.New("display_name is too long")
	}

	return nil
}

// TODO
/*
pub fn password<S: std::hash::BuildHasher>(
    basic_passwords: HashSet<String, S>,
    password: &str,
) -> Result<(), BloomError> {
    let length = password.len();

    if length < 8 {
        return Err(BloomError::Validation(
            "Password must be longer than 7 characters".to_string(),
        ));
    } else if length > 128 {
        return Err(BloomError::Validation(
            "Password must be shorter than 128 characters".to_string(),
        ));
    } else if basic_passwords
        .iter()
        .any(|basic_password| basic_password == password)
    {
        return Err(BloomError::Validation("Password is too weak".to_string()));
    }

    return Ok(());
}
*/
func validatePassword(password string, basicPassword map[string]bool) error {
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
	return nil
}

// TODO
/*
pub fn username(username: &str) -> Result<(), BloomError> {

    let invalid_usernames = vec![
        "admin",
        "sysy",
        "asministrator",
        "bloom",
        "bloom42",
        "support",
        "help",
        "settings",
        "admin1",
        "security",
        "profile",
        "42bloom",
        "sylvain.kerkour",
        "sylvainkerkour",
        "kerkour.sylvain",
        "kerkoursylvain",
    ];

    if invalid_usernames.contains(&username) {
        return Err(BloomError::Validation("username is not valid".to_string()));
    }

    if username.contains("administrator") {
        return Err(BloomError::Validation("username is not valid".to_string()));
    }


    return Ok(());
}
*/

var isAlphaNumeric = regexp.MustCompile(`^[a-z0-9]+$`).MatchString

func validateUsername(username string) error {
	usernameLength := len(username)

	if usernameLength == 0 {
		return errors.New("username cannot be empty")
	}

	if usernameLength < consts.USERNAME_MIN_LENGTH {
		return errors.New("username is too short")
	}

	if usernameLength > consts.USERNAME_MAX_LENGTH {
		return errors.New("username is too long")
	}

	if username != strings.ToLower(username) {
		return errors.New("username must be lowercase")
	}

	if !isAlphaNumeric(username) {
		return errors.New("username must contains only alphanumeric characters")
	}

	if strings.Contains(username, "admin") || stringSliceContains(consts.INVALID_USERNAMES, username) {
		return errors.New("username is not valid")
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

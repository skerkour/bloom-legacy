package validator

import (
	"errors"
	"fmt"
	"regexp"
	"strings"

	"github.com/asaskevich/govalidator"
	"gitlab.com/bloom42/bloom/common/consts"
)

func UserFirstName(firstName string) error {
	firstNameLen := len(firstName)

	if firstNameLen == 0 {
		return errors.New("first_name cannot be empty")
	}

	if firstNameLen > consts.FIRST_NAME_MAX_LENGTH {
		return errors.New("first_name is too long")
	}

	return nil
}

func UserLastName(lastName string) error {
	lastNameLen := len(lastName)

	if lastNameLen == 0 {
		return errors.New("last_name cannot be empty")
	}

	if lastNameLen > consts.LAST_NAME_MAX_LENGTH {
		return errors.New("last_name is too long")
	}

	return nil
}

func UserBio(bio string) error {
	if len(bio) > consts.BIO_MAX_LENGTH {
		return errors.New("bio is too long")
	}

	return nil
}

func UserDisplayName(displayName string) error {
	displayNameLen := len(displayName)

	if displayNameLen == 0 {
		return errors.New("display_name cannot be empty")
	}

	if displayNameLen > consts.DISPLAY_NAME_MAX_LENGTH {
		return errors.New("display_name is too long")
	}

	return nil
}

func UserPassword(password string, basicPassword map[string]bool) error {
	passwordLength := len(password)

	if passwordLength < consts.PASSWORD_MAX_LENGTH {
		return fmt.Errorf("Password must be longer than %d characters", consts.PASSWORD_MAX_LENGTH-1)
	}

	if passwordLength > consts.PASSWORD_MAX_LENGTH {
		return fmt.Errorf("Password must be shorter than %d characters", consts.PASSWORD_MAX_LENGTH)
	}

	if _, ok := basicPassword[password]; ok {
		return errors.New("Password is too weak")
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
func UserEmail(email string, disposableEmailDomains map[string]bool) error {
	if !govalidator.IsEmail(email) {
		return errors.New("email is not valid")
	}
	return nil
}

var isAlphaNumeric = regexp.MustCompile(`^[a-z0-9]+$`).MatchString

func UserUsername(username string) error {
	usernameLength := len(username)

	if usernameLength == 0 {
		return errors.New("username cannot be empty")
	}

	if usernameLength < consts.USERNAME_MIN_LENGTH {
		return fmt.Errorf("username must be longer than %d characters", consts.USERNAME_MIN_LENGTH-1)
	}

	if usernameLength > consts.USERNAME_MAX_LENGTH {
		return fmt.Errorf("username must be longer than %d characters", consts.USERNAME_MAX_LENGTH)
	}

	if username != strings.ToLower(username) {
		return errors.New("username must be lowercase")
	}

	if !isAlphaNumeric(username) {
		return errors.New("username must contains only alphanumeric characters")
	}

	if strings.Contains(username, "admin") ||
		strings.Contains(username, "bloom") ||
		strings.HasSuffix(username, "bot") ||
		stringSliceContains(consts.INVALID_USERNAMES, username) {
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

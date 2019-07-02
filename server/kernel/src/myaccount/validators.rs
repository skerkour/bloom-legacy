use crate::{error::KernelError, myaccount};
use regex::Regex;
use std::collections::HashSet;

pub fn first_name(first_name: &str) -> Result<(), KernelError> {
    if first_name.is_empty() {
        return Err(KernelError::Validation(
            "first_name cannot be empty".to_string(),
        ));
    }

    if first_name.len() > 64 {
        return Err(KernelError::Validation(
            "first_name is too long".to_string(),
        ));
    }

    return Ok(());
}

pub fn last_name(last_name: &str) -> Result<(), KernelError> {
    if last_name.is_empty() {
        return Err(KernelError::Validation(
            "last_name cannot be empty".to_string(),
        ));
    }

    if last_name.len() > 64 {
        return Err(KernelError::Validation("last_name is too long".to_string()));
    }

    return Ok(());
}

pub fn password(basic_passwords: HashSet<String>, password: &str) -> Result<(), KernelError> {
    let length = password.len();

    if length < 8 {
        return Err(KernelError::Validation(
            "Password must be longer than 7 characters".to_string(),
        ));
    } else if length > 128 {
        return Err(KernelError::Validation(
            "Password must be shorter than 128 characters".to_string(),
        ));
    } else if basic_passwords
        .iter()
        .any(|basic_password| basic_password == password)
    {
        return Err(KernelError::Validation("Password is too weak".to_string()));
    }

    return Ok(());
}

pub fn email(disposable_emails: HashSet<String>, email: &str) -> Result<(), KernelError> {
    if email.is_empty() || !email.contains('@') {
        return Err(KernelError::Validation("email is not valid".to_string()));
    }

    let parts: Vec<&str> = email.split("@").collect();

    if parts.len() != 2 {
        return Err(KernelError::Validation("email is not valid".to_string()));
    }

    let user_part = parts[1];
    let domain_part = parts[0];

    if user_part.is_empty() || domain_part.is_empty() {
        return Err(KernelError::Validation("email is not valid".to_string()));
    }

    if email.trim() != email {
        return Err(KernelError::Validation(
            "email must not contains whitesapces".to_string(),
        ));
    }

    if email.len() > 128 {
        return Err(KernelError::Validation("email is too long".to_string()));
    }

    let user_re = Regex::new(r"^(?i)[a-z0-9.!#$%&'*+/=?^_`{|}~-]+\z")
        .expect("error compiling user email regex");
    let domain_re = Regex::new(
        r"(?i)^[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)*$",
    )
    .expect("error compiling domain email regex");

    if !user_re.is_match(user_part) {
        return Err(KernelError::Validation("email is not valid".to_string()));
    }
    if !domain_re.is_match(domain_part) {
        return Err(KernelError::Validation("email is not valid".to_string()));
    }

    if disposable_emails.contains(&domain_part.to_string()) {
        return Err(KernelError::Validation(
            "email domain is not valid".to_string(),
        ));
    }

    return Ok(());
}

pub fn username(username: &str) -> Result<(), KernelError> {
    if username.is_empty() {
        return Err(KernelError::Validation(
            "username cannot be empty".to_string(),
        ));
    }

    if username.len() < 4 {
        return Err(KernelError::Validation("username is too short".to_string()));
    }

    if username.len() > 16 {
        return Err(KernelError::Validation("username is too long".to_string()));
    }

    if username.to_lowercase() != username {
        return Err(KernelError::Validation(
            "username must be lowercase".to_string(),
        ));
    }

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
        return Err(KernelError::Validation("username is not valid".to_string()));
    }

    if username.contains("administrator") {
        return Err(KernelError::Validation("username is not valid".to_string()));
    }

    if !username.chars().all(char::is_alphanumeric) || !username.is_ascii() {
        return Err(KernelError::Validation(
            "username must contains only alphanumeric characters".to_string(),
        ));
    }

    return Ok(());
}

pub fn bio(bio: &str) -> Result<(), KernelError> {
    if bio.len() > myaccount::BIO_MAX_LENGTH {
        return Err(KernelError::Validation("bio is too long".to_string()));
    }

    return Ok(());
}

pub fn display_name(display_name: &str) -> Result<(), KernelError> {
    if display_name.is_empty() {
        return Err(KernelError::Validation(
            "display_name cannot be empty".to_string(),
        ));
    }

    if display_name.len() > myaccount::DISPLAY_NAME_MAX_LENGTH {
        return Err(KernelError::Validation(
            "display_name is too long".to_string(),
        ));
    }

    return Ok(());
}

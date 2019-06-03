use crate::error::KernelError;
use std::collections::HashSet;


pub fn first_name(first_name: &str) -> Result<(), KernelError> {
    if first_name.is_empty() {
        return Err(KernelError::Validation("first_name cannot be empty".to_string()));
    }

    return Ok(());
}


pub fn last_name(last_name: &str) -> Result<(), KernelError> {
    if last_name.is_empty() {
        return Err(KernelError::Validation("last_name cannot be empty".to_string()));
    }

    return Ok(());
}

pub fn password(password: &str) -> Result<(), KernelError> {
    let length = password.len();

    if length < 8 {
        return Err(KernelError::Validation("password must be longer than 7 characters".to_string()));
    } else if length > 128 {
        return Err(KernelError::Validation("password must be shorter than 128 characters".to_string()));
    }

    return Ok(());
}


// TODO
pub fn email(_email: &str) -> Result<(), KernelError> {
    return Ok(());
}

pub fn username(username: &str) -> Result<(), KernelError> {
    if username.is_empty() {
        return Err(KernelError::Validation("username cannot be empty".to_string()));
    }

    if username.len() < 4 {
        return Err(KernelError::Validation("username is too short".to_string()));
    }

    if username.len() > 16 {
        return Err(KernelError::Validation("username is too long".to_string()));
    }

    if username.to_lowercase() != username {
        return Err(KernelError::Validation("username must be lowercase".to_string()));
    }

    let mut invalid_usernames = HashSet::new();
    invalid_usernames.insert("admin");
    invalid_usernames.insert("sysy");
    invalid_usernames.insert("asministrator");
    invalid_usernames.insert("bloom");
    invalid_usernames.insert("bloom42");

    if invalid_usernames.contains(username) {
        return Err(KernelError::Validation("username is not valid".to_string()));
    }

    return Ok(());
}

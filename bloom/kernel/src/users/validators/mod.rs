use crate::error::KernelError;


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
pub fn email(email: &str) -> Result<(), KernelError> {
    return Ok(());
}


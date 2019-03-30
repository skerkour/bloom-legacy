use crate::error::KernelError;


pub fn title(title: &str) -> Result<(), KernelError> {
    if title.len() > 256 {
        return Err(KernelError::Validation("Title's length must be lower than 257 characters".to_string()));
    }

    return Ok(());
}


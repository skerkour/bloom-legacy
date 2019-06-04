use kernel::KernelError;


pub fn title(title: &str) -> Result<(), KernelError> {
    if title.len() > 64 {
        return Err(KernelError::Validation("Title length is too long".to_string()));
    }

    return Ok(());
}


use kernel::KernelError;

pub fn title(title: &str) -> Result<(), KernelError> {
    if title.len() > 64 {
        return Err(KernelError::Validation(
            "Title length is too long".to_string(),
        ));
    }

    return Ok(());
}

pub fn body(body: &str) -> Result<(), KernelError> {
    if body.len() > 8192 {
        return Err(KernelError::Validation(
            "Body length is too long".to_string(),
        ));
    }

    return Ok(());
}

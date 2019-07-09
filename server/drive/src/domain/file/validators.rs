use kernel::KernelError;

// TODO
pub fn name(file_name: &str) -> Result<(), KernelError> {
    if file_name.len() > 128 {
        return Err(KernelError::Validation("file name is too long".to_string()));
    }

    if file_name == crate::BLOOM_ROOT_NAME {
        return Err(KernelError::Validation(
            "file name is not valid".to_string(),
        ));
    }

    return Ok(());
}

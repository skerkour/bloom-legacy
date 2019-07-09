use kernel::KernelError;

// TODO
pub fn name(file_name: &str) -> Result<(), KernelError> {
    if file_name.len() > 128 {
        return Err(KernelError::Validation("file name is too long".to_string()));
    }

    return Ok(());
}

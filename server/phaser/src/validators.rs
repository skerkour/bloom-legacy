use kernel::KernelError;


pub fn scan_name(name: &str) -> Result<(), KernelError> {
    if name.len() > 256 {
        return Err(KernelError::Validation("Name's length must be lower than 257 characters".to_string()));
    }

    return Ok(());
}

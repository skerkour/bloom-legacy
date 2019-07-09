use kernel::KernelError;

pub fn name(album_name: &str) -> Result<(), KernelError> {
    if album_name.is_empty() {
        return Err(KernelError::Validation(
            "album name cannot be empty".to_string(),
        ));
    }
    if album_name.trim() != album_name {
        return Err(KernelError::Validation(
            "album name is not valid".to_string(),
        ));
    }

    if album_name.len() < 4 {
        return Err(KernelError::Validation(
            "album name is too short".to_string(),
        ));
    }

    if album_name.len() > 64 {
        return Err(KernelError::Validation(
            "album name is too long".to_string(),
        ));
    }

    return Ok(());
}

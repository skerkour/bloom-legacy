use kernel::KernelError;

pub fn name(playlist_name: &str) -> Result<(), KernelError> {
    if playlist_name.is_empty() {
        return Err(KernelError::Validation(
            "playlist name cannot be empty".to_string(),
        ));
    }
    if playlist_name.trim() != playlist_name {
        return Err(KernelError::Validation(
            "playlist name is not valid".to_string(),
        ));
    }

    if playlist_name.len() < 4 {
        return Err(KernelError::Validation(
            "playlist name is too short".to_string(),
        ));
    }

    if playlist_name.len() > 64 {
        return Err(KernelError::Validation(
            "playlist name is too long".to_string(),
        ));
    }
    return Ok(());
}

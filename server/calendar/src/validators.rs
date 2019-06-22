use kernel::KernelError;

pub fn date(start_at: chrono::DateTime<chrono::Utc>, end_at: chrono::DateTime<chrono::Utc>) -> Result<(), KernelError> {
    if end_at < start_at {
        return Err(KernelError::Validation("end_at must be after start_at".to_string()));
    }

    Ok(())
}

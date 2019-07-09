use kernel::KernelError;

pub fn dates(
    start_at: chrono::DateTime<chrono::Utc>,
    end_at: chrono::DateTime<chrono::Utc>,
) -> Result<(), KernelError> {
    if end_at < start_at {
        return Err(KernelError::Validation(
            "end_at must be after start_at".to_string(),
        ));
    }

    Ok(())
}

pub fn title(title: &str) -> Result<(), KernelError> {
    if title.len() > 256 {
        return Err(KernelError::Validation("title is too long".to_string()));
    }

    return Ok(());
}

pub fn description(description: &str) -> Result<(), KernelError> {
    if description.len() > 2048 {
        return Err(KernelError::Validation(
            "description is too long".to_string(),
        ));
    }

    return Ok(());
}

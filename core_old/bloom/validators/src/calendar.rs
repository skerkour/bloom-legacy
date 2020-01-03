use bloom_error::BloomError;

pub fn event_dates(
    start_at: chrono::DateTime<chrono::Utc>,
    end_at: chrono::DateTime<chrono::Utc>,
) -> Result<(), BloomError> {
    if end_at < start_at {
        return Err(BloomError::Validation(
            "end_at must be after start_at".to_string(),
        ));
    }

    Ok(())
}

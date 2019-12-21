use bloom_error::BloomError;
use url::Url;

pub fn bitflow_secret(secret: &str) -> Result<(), BloomError> {
    if secret.len() < 64 {
        return Err(BloomError::Validation(
            "Bitflow secret length must be at least 64.".to_string(),
        ));
    }

    return Ok(());
}

pub fn host(host: &str) -> Result<(), BloomError> {
    // host have scheme
    let parsed_host = Url::parse(host).map_err(|err| BloomError::UrlParseError(err.to_string()))?;
    if parsed_host.scheme().is_empty() {
        return Err(BloomError::Validation(
            "Host musht have a URL scheme. eg. http://localhost:8080.".to_string(),
        ));
    }

    return Ok(());
}

pub fn db_url(url: &str) -> Result<(), BloomError> {
    if url.is_empty() {
        return Err(BloomError::Validation(
            "Database URL must not be empty.".to_string(),
        ));
    }

    return Ok(());
}

use failure::Fail;

#[derive(Debug, Fail)]
pub enum BloomError {
    #[fail(display = "Reqwest: {}", 0)]
    Reqwest(reqwest::Error),

    #[fail(display = "Crypto42: {:?}", 0)]
    Crypto42(crypto42::Error),
}

// TODO: improve...
impl From<BloomError> for bloom_messages::kernel::Error {
    fn from(err: BloomError) -> Self {
        let code = match &err {
            _ => "INTERNAL",
        }
        .to_string();
        bloom_messages::kernel::Error {
            code,
            message: format!("{}", err),
        }
    }
}

impl From<reqwest::Error> for BloomError {
    fn from(err: reqwest::Error) -> Self {
        BloomError::Reqwest(err)
    }
}

impl From<crypto42::Error> for BloomError {
    fn from(err: crypto42::Error) -> Self {
        BloomError::Crypto42(err)
    }
}

// TODO(z0mbie42):
// * from other error types (crypto42, reqwest...)
// * into message

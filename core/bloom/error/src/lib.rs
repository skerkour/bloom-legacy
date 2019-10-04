use failure::Fail;

mod code;

pub use code::ErrorCode;

#[derive(Debug, Fail)]
pub enum BloomError {
    #[fail(display = "Reqwest: {}", 0)]
    Reqwest(reqwest::Error),

    #[fail(display = "Crypto42: {:?}", 0)]
    Crypto42(crypto42::Error),

    #[fail(display = "Unknown message type")]
    UnknownMessageType,
}

// TODO: improve...
impl From<BloomError> for bloom_messages::kernel::Error {
    fn from(err: BloomError) -> Self {
        let code = match &err {
            BloomError::UnknownMessageType => ErrorCode::UKNOWN_MESSAGE_TYPE,
            _ => ErrorCode::INTERNAL,
        };
        let message = match code {
            ErrorCode::INTERNAL => "Internal error".to_string(),
            _ => format!("{}", err),
        };

        bloom_messages::kernel::Error {
            code: code.to_string(),
            message,
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

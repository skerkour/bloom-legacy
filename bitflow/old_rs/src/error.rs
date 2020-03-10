use failure::Fail;
use std::io;

#[derive(Debug, Fail)]
pub enum BitflowError {
    #[fail(display = "Io: {:?}", 0)]
    Io(io::Error),

    #[fail(display = "Http: {:?}", 0)]
    Reqwest(reqwest::Error),

    #[fail(display = "Walkdir: {:?}", 0)]
    Walkdir(walkdir::Error),

    #[fail(display = "Internal: {:?}", 0)]
    Internal(String),
}

impl From<std::io::Error> for BitflowError {
    fn from(err: std::io::Error) -> Self {
        return BitflowError::Io(err);
    }
}

impl From<reqwest::Error> for BitflowError {
    fn from(err: reqwest::Error) -> Self {
        return BitflowError::Reqwest(err);
    }
}

impl From<walkdir::Error> for BitflowError {
    fn from(err: walkdir::Error) -> Self {
        return BitflowError::Walkdir(err);
    }
}

pub enum ErrorCode {
    INTERNAL,
    UKNOWN_MESSAGE_TYPE,
    FORBIDDEN,
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
